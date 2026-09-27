// T-15 wave 0 · B1 — пилот токен-гигиены: обрезка выводов инструментов.
// Карточка: docs/tasks/T-15-mcp-ready-process/wave0-token-hygiene.md §3.2 (B1).
// План/чек-лист: docs/tasks/T-15-mcp-ready-process/wave0-plan.md (W0-i2).
//
// V2 Plugin API (OpenCode CLI 2.0.x): Plugin.define({ id, setup }); setup(ctx)
// получает контекст (ctx.tool, ctx.storage, ctx.session, …). Пакет типов —
// `@opencode/plugin@2.0.18`, установлен в `.opencode/` (резолвится из
// `.opencode/node_modules`).
// История 2026-09-28: (1) V1-хуки → V2-форма; (2) промежуточно —
// import-free default-объект (импорт не резолвился до установки пакета);
// (3) канонический импорт возвращён после установки `@opencode/plugin@2.0.18`
// (V1-пакет `@opencode-ai/plugin` удалён). Разбор — в ленте
// .opencode/mail/service-mcp-ready.md. Примечание: правки плагина делать
// одним атомарным write — watcher перезагружает файл на каждое изменение и
// ловит промежуточные (неполные) состояния.
//
// Права не меняет, схемы инструментов не трогает (это B2, W0-i3). Откат —
// переименовать/удалить файл (автозагрузка .opencode/plugins/**; watcher
// следит за файлом, при необходимости — рестарт сервиса OpenCode).
//
// Счётчики — ctx.storage (персистентный JSON плагина, ключ "stats"); вывод —
// console.log сервера на каждый срез.
//
// Соседство с B2 (W0-i3): B2 будет добавлен в ЭТОТ же файл —
// ctx.session.hook("context") (агентский цикл), удаление ключей event.tools
// по префиксам; см. wave0-plan.md, «Общие решения».

import { Plugin } from "@opencode/plugin";

/** Инструменты-исключения из среза (передача контекста под-ролям). */
const EXCLUDED_TOOLS = new Set<string>(["subagent", "task"]);

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

interface SliceOutcome {
  text: string;
  changed: boolean;
  slices: number;
  trimmedBytes: number;
}

/** Обрезка одного текста: голова+хвост, важные строки, ANSI-чистка. */
const sliceText = (raw: string, tool: string): SliceOutcome => {
  const text = stripAnsi(raw);
  if (text.length <= OUTPUT_LIMIT) {
    if (text === raw)
      return { text: raw, changed: false, slices: 0, trimmedBytes: 0 };
    return { text, changed: true, slices: 0, trimmedBytes: 0 };
  }

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
    tool === "read" ? "остаток опущен (offset/limit)" : "остаток опущен";
  const marker = `\n…[token-guard] срез: опущено ~${omittedBytes} байт из ${bytesBefore}; ${note}`;
  const recoveredBlock =
    recovered.length > 0
      ? `\n…[token-guard] важные строки из опущенного (${recovered.length}):\n` +
        recovered.join("\n")
      : "";

  return {
    text: head + marker + recoveredBlock + "\n…[конец среза]\n" + tail,
    changed: true,
    slices: 1,
    trimmedBytes: omittedBytes,
  };
};

interface Stats {
  slices: number;
  bytesTrimmed: number;
  byTool: Record<string, { slices: number; bytesTrimmed: number }>;
}

export default Plugin.define({
  id: "token-guard",
  async setup(ctx) {
    const STATS_KEY = "stats";
    const emptyStats = (): Stats => ({ slices: 0, bytesTrimmed: 0, byTool: {} });

    const readStats = async (): Promise<Stats> => {
      try {
        const parsed: any = await ctx.storage.get(STATS_KEY);
        if (!parsed || typeof parsed !== "object") return emptyStats();
        return {
          slices: Number(parsed.slices) || 0,
          bytesTrimmed: Number(parsed.bytesTrimmed) || 0,
          byTool:
            parsed.byTool && typeof parsed.byTool === "object"
              ? parsed.byTool
              : {},
        };
      } catch {
        return emptyStats();
      }
    };

    await ctx.tool.hook("execute.after", async (event: any) => {
      try {
        if (event?.status !== "completed") return;
        // Исключённые инструменты (передача контекста под-роли) не режем.
        if (typeof event.tool === "string" && EXCLUDED_TOOLS.has(event.tool))
          return;
        const result: any = event.result;
        if (!result) return;

        let content = result.content;
        let output = result.output;
        let slices = 0;
        let trimmedBytes = 0;
        let changed = false;

        if (typeof content === "string") {
          const o = sliceText(content, event.tool);
          if (o.changed) {
            content = o.text;
            slices += o.slices;
            trimmedBytes += o.trimmedBytes;
            changed = true;
          }
        } else if (Array.isArray(content)) {
          const next = content.map((part: any) => {
            if (part && part.type === "text" && typeof part.text === "string") {
              const o = sliceText(part.text, event.tool);
              if (o.changed) {
                slices += o.slices;
                trimmedBytes += o.trimmedBytes;
                changed = true;
                return { ...part, text: o.text };
              }
            }
            return part;
          });
          if (changed) content = next;
        }

        // На случай текстового структурированного вывода.
        if (typeof output === "string") {
          const o = sliceText(output, event.tool);
          if (o.changed) {
            output = o.text;
            slices += o.slices;
            trimmedBytes += o.trimmedBytes;
            changed = true;
          }
        }

        if (!changed) return;
        // Подмена результата до отправки в контекст модели.
        event.result = { ...result, content, output };

        // Только ANSI-чистка (без среза) — счётчики не трогаем.
        if (slices === 0) return;

        const stats = await readStats();
        stats.slices += slices;
        stats.bytesTrimmed += trimmedBytes;
        const per = stats.byTool[event.tool] ?? { slices: 0, bytesTrimmed: 0 };
        per.slices += slices;
        per.bytesTrimmed += trimmedBytes;
        stats.byTool[event.tool] = per;
        await ctx.storage.set(STATS_KEY, stats);
        console.log(
          `[token-guard] tool=${event.tool} slices=${slices} trimmed=${trimmedBytes}B total=${stats.bytesTrimmed}B`,
        );
      } catch (e) {
        // Никакого «глушения» роли: при любой ошибке результат остаётся как есть.
        console.warn("[token-guard] execute.after failed, result untouched:", e);
      }
    });
  },
});
