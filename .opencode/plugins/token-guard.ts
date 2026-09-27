// T-15 wave 0 · B1 — пилот токен-гигиены: обрезка выводов инструментов.
// Карточка: docs/tasks/T-15-mcp-ready-process/wave0-token-hygiene.md §3.2 (B1).
// План/чек-лист: docs/tasks/T-15-mcp-ready-process/wave0-plan.md (W0-i2).
//
// Права не меняет, схемы инструментов не трогает (это B2, W0-i3). Откат —
// переименовать/удалить файл (автозагрузка .opencode/plugins/**) + рестарт
// сервиса OpenCode.
//
// Механизм (V2 Plugin API): хук `tool.execute.after` — мутация поля `output`
// заменяет вывод инструмента, отправляемый в контекст модели. В текущем SDK
// у плагина нет общего персистентного storage (PluginInput: client/project/
// directory/worktree/$) — поэтому счётчики экономии пишутся в локальный
// JSON-файл `.opencode/plugins/token-guard.stats.json` (вне коммита);
// вывод счётчиков — событие `session.idle` (в stdout сервера OpenCode).
// Отступление от §3.2 карточки («ctx.storage») зафиксировано в ленте T-15.
//
// Соседство с B2 (W0-i3): B2 будет добавлен в ЭТОТ же файл (хук
// `experimental.session.compacting`) — см. wave0-plan.md, «Общие решения».

import * as fs from "node:fs";
import * as path from "node:path";
import type { Plugin, Hooks } from "@opencode-ai/plugin";

/** Инструменты-исключения из среза (низкорисковый консервативный список;
 *  расширяется по данным счётчиков на W0-i4). */
const EXCLUDED_TOOLS = new Set<string>(["task"]);

// ── Настройки (настраиваемая константа; карточка §3.2) ──────────────────────

/** Лимит вывода инструмента, в символах (UTF-16 code units). ~12 КБ. */
const OUTPUT_LIMIT = 12_000;
/** Сколько символов сохранять из «головы» при срезе. */
const HEAD_KEEP = Math.floor(OUTPUT_LIMIT * 0.6); // 7200
/** И из «хвоста». */
const TAIL_KEEP = OUTPUT_LIMIT - HEAD_KEEP; // 4800
/** Максимум «важных» строк, добавляемых после среза. */
const MAX_IMPORTANT_LINES = 200;

/** Строки, которые всегда сохраняем при срезе (регистронезависимо). */
const IMPORTANT_RE = /\b(error|warning|FAILED|failed|passed)\b/i;

/** ANSI-escape-последовательности (в т.ч. OSC-гиперссылки). */
const ANSI_RE =
  // eslint-disable-next-line no-control-regex
  /\x1b\[[0-9;?]*[ -/]*[@-~]|\x1b\][^\x07\x1b]*(?:\x07|\x1b\\)|\x1b[@-Z\\-_]/g;

const stripAnsi = (s: string): string => s.replace(ANSI_RE, "");

interface SliceStats {
  slices: number;
  bytesTrimmed: number;
  byTool: Record<string, { slices: number; bytesTrimmed: number }>;
}

const STATS_FILE = "token-guard.stats.json";
const emptyStats = (): SliceStats => ({ slices: 0, bytesTrimmed: 0, byTool: {} });

export default async function tokenGuard({ directory }: any): Promise<Plugin> {
  const statsPath = path.join(directory ?? ".", ".opencode", "plugins", STATS_FILE);

  const readStats = (): SliceStats => {
    try {
      const parsed = JSON.parse(fs.readFileSync(statsPath, "utf8"));
      return {
        slices: Number(parsed?.slices) || 0,
        bytesTrimmed: Number(parsed?.bytesTrimmed) || 0,
        byTool:
          parsed?.byTool && typeof parsed.byTool === "object" ? parsed.byTool : {},
      };
    } catch {
      return emptyStats();
    }
  };

  // Атомарная перезапись снимка (tmp + rename), чтобы не оставить обрезанный
  // JSON при параллельных хуках/процессах.
  const writeStats = (s: SliceStats): void => {
    try {
      const tmp = `${statsPath}.${process.pid}.tmp`;
      fs.writeFileSync(tmp, JSON.stringify(s));
      fs.renameSync(tmp, statsPath);
    } catch (e) {
      console.warn("[token-guard] stats write failed:", e);
    }
  };

  return {
    async "tool.execute.after"(
      input: Parameters<NonNullable<Hooks["tool.execute.after"]>>[0],
      output: Parameters<NonNullable<Hooks["tool.execute.after"]>>[1],
    ) {
      try {
        // Исключённые инструменты (передача контекста под-роли) не режем.
        if (typeof input?.tool === "string" && EXCLUDED_TOOLS.has(input.tool))
          return;
        const raw = output?.output;
        if (typeof raw !== "string") return;
        const text = stripAnsi(raw);
        // Основной путь: короткий чистый вывод не трогаем вовсе.
        if (text.length <= OUTPUT_LIMIT && text === raw) return;

        const needSlice = text.length > OUTPUT_LIMIT;
        if (!needSlice) {
          // только чистка ANSI, без среза и пометок
          output.output = text;
          return;
        }

        // Срез голова+хвост.
        const head = text.slice(0, HEAD_KEEP);
        const tail = text.slice(-TAIL_KEEP);
        const middle = text.slice(HEAD_KEEP, text.length - TAIL_KEEP);

        // Важные строки из опущенной середины — дословно, в исходном порядке,
        // без дублей того, что уже попало в голову/хвост.
        const keptSet = new Set<string>();
        for (const line of head.split("\n"))
          if (IMPORTANT_RE.test(line)) keptSet.add(line);
        for (const line of tail.split("\n"))
          if (IMPORTANT_RE.test(line)) keptSet.add(line);
        const recovered: string[] = [];
        for (const line of middle.split("\n")) {
          if (recovered.length >= MAX_IMPORTANT_LINES) break;
          if (IMPORTANT_RE.test(line) && !keptSet.has(line)) {
            keptSet.add(line);
            recovered.push(line);
          }
        }

        const bytesBefore = Buffer.byteLength(raw, "utf8");
        const bytesAfter = Buffer.byteLength(head + tail, "utf8");
        const omittedBytes = Math.max(0, bytesBefore - bytesAfter);
        const note =
          input?.tool === "read"
            ? "остаток опущен (offset/limit)"
            : "остаток опущен";
        const marker = `\n…[token-guard] срез: опущено ~${omittedBytes} байт из ${bytesBefore}; ${note}`;
        const recoveredBlock =
          recovered.length > 0
            ? `\n…[token-guard] важные строки из опущенного (${recovered.length}):\n` +
              recovered.join("\n")
            : "";

        output.output = head + marker + recoveredBlock + "\n…[конец среза]\n" + tail;

        // Счётчики экономии → локальный снимок (см. шапку файла).
        const stats = readStats();
        stats.slices += 1;
        stats.bytesTrimmed += omittedBytes;
        const per = stats.byTool[input.tool] ?? { slices: 0, bytesTrimmed: 0 };
        per.slices += 1;
        per.bytesTrimmed += omittedBytes;
        stats.byTool[input.tool] = per;
        writeStats(stats);
      } catch (e) {
        // Никакого «глушения» роли: при любой ошибке вывод остаётся как есть.
        console.warn("[token-guard] execute.after failed, output untouched:", e);
      }
    },

    // Печать итога счётчиков при простое сессии — в лог сервера OpenCode
    // (основной путь вывода — /token-guard/stats, чтение файла-снимка).
    async event({ event }: { event: any }) {
      try {
        if (event?.type === "session.idle") {
          const s = readStats();
          console.log(
            `[token-guard] counters: slices=${s.slices}, bytesTrimmed=${s.bytesTrimmed}`,
          );
        }
      } catch {
        /* non-fatal */
      }
    },
  } satisfies Hooks;
}
