// T-15 wave 0 · B1+B2 — пилот токен-гигиены: B1 обрезка выводов инструментов;
// B2 снятие tool-схем по именам агентов.
// Карточка: docs/tasks/T-15-mcp-ready-process/wave0-token-hygiene.md §3.2 (B1/B2).
// План/чек-лист: docs/tasks/T-15-mcp-ready-process/wave0-plan.md (W0-i2, W0-i3).
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
// Права не меняет; B2 (снятие tool-схем по именам агентов) — профиль-флаг
// (D94): константа B2_PROFILE ∈ "off" | "codemode" | "mcp"; по умолчанию "off"
// (таблица пуста, ничего не снимается). Включение — смена константы; при прямой
// экспозиции MCP (codemode=false) — "mcp". permissions и канон не правятся.
// Откат — переименовать/удалить файл (автозагрузка .opencode/plugins/**;
// watcher следит за файлом, при необходимости — рестарт сервиса OpenCode).
//
// Счётчики — ctx.storage (персистентный JSON плагина): B1 — ключ "stats",
// B2 — ключ "b2stats"; вывод — console.log сервера.
//
// W0-i3 (эксперимент §3.5, 2026-09-27): статический deny снимает схему
// инструмента из запроса (подтверждено на `skill`/`webfetch`/`execute` в
// репозитории и на `credo_check_create` в изолированном прогоне с
// codemode=false); плагин снимает ключи по префиксам. В текущей конфигурации
// репозитория (codemode=true по умолчанию) MCP-схемы не являются ключами
// event.tools — активный профиль "off" (D94, 2026-10-03): правила обоих
// профилей внесены, включение — сменой B2_PROFILE при прямой экспозиции MCP.

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

// ── B2 (W0-i3; профиль-флаг D94): снятие tool-схем по именам агентов ────────
//
// Правила §3.2 сверены с брифами и review.md (лента service-mcp-ready,
// 2026-09-27): docs-writer/git — без `rust-analyzer*`/`credo*`; analyst — без
// `rust-analyzer*`; lead — не трогать. Префиксы покрывают оба написания сервера
// (`rust-analyzer` / `rust_analyzer`) и MCP-ключи вида `<server>_<tool>`
// (изолированный прогон: `credo_check_create`). Активная таблица выбирается
// константой B2_PROFILE; "off" — no-op.
// Хук `session.context` выполняется только для агентского цикла (primary);
// у compaction/title/generate собственные хуки, поэтому условие «только
// primary» задано самой регистрацией (kind в событии не приходит).

/** Активный профиль B2 (D94): "off" — выключено, "codemode"/"mcp" — таблица. */
const B2_PROFILE: "off" | "codemode" | "mcp" = "off";

/** Профиль Code Mode (инструменты/MCP внутри `execute`). */
const B2_PREFIXES_CODEMODE: Record<string, readonly string[]> = {
  "docs-writer": ["rust-analyzer", "rust_analyzer", "credo"],
  git: ["rust-analyzer", "rust_analyzer", "credo"],
  analyst: ["rust-analyzer", "rust_analyzer"],
};

/** Профиль прямой экспозиции MCP (codemode=false). */
const B2_PREFIXES_MCP: Record<string, readonly string[]> = {
  "docs-writer": ["rust-analyzer", "rust_analyzer", "credo"],
  git: ["rust-analyzer", "rust_analyzer", "credo"],
  analyst: ["rust-analyzer", "rust_analyzer"],
};

/** Таблицы профилей. */
const B2_PROFILES: Record<string, Record<string, readonly string[]>> = {
  codemode: B2_PREFIXES_CODEMODE,
  mcp: B2_PREFIXES_MCP,
};

/** Префиксы активного профиля ("" при "off" — ничего не снимается). */
const activeB2Prefixes = (): Record<string, readonly string[]> =>
  B2_PROFILES[B2_PROFILE] ?? {};

interface B2Stats {
  removed: number;
  bytesTrimmed: number;
  byAgent: Record<string, { removed: number; bytesTrimmed: number }>;
}

interface Stats {
  slices: number;
  bytesTrimmed: number;
  byTool: Record<string, { slices: number; bytesTrimmed: number }>;
}

export default Plugin.define({
  id: "token-guard",
  async setup(ctx) {
    const STATS_KEY = "stats";
    const B2_STATS_KEY = "b2stats";
    const emptyStats = (): Stats => ({ slices: 0, bytesTrimmed: 0, byTool: {} });
    const emptyB2Stats = (): B2Stats => ({
      removed: 0,
      bytesTrimmed: 0,
      byAgent: {},
    });

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

    const readB2Stats = async (): Promise<B2Stats> => {
      try {
        const parsed: any = await ctx.storage.get(B2_STATS_KEY);
        if (!parsed || typeof parsed !== "object") return emptyB2Stats();
        return {
          removed: Number(parsed.removed) || 0,
          bytesTrimmed: Number(parsed.bytesTrimmed) || 0,
          byAgent:
            parsed.byAgent && typeof parsed.byAgent === "object"
              ? parsed.byAgent
              : {},
        };
      } catch {
        return emptyB2Stats();
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

    // B2: удаление ненужных роли tool-схем из исходящего запроса (только
    // агентский цикл — context; права и канон не меняются).
    await ctx.session.hook("context", async (event: any) => {
      try {
        // Страховка на будущее: в доках V2 «context» — это primary; поле
        // kind в событии не приходит, но если появится — чужое не трогаем.
        if (event?.kind && event.kind !== "primary") return;
        const tools: Record<string, unknown> | undefined = event?.tools;
        const agent = typeof event?.agent === "string" ? event.agent : "";
        const prefixes = activeB2Prefixes()[agent];
        if (!tools || !prefixes) return;

        let removed = 0;
        let bytesTrimmed = 0;
        for (const key of Object.keys(tools)) {
          if (!prefixes.some((p) => key.startsWith(p))) continue;
          try {
            bytesTrimmed += JSON.stringify(tools[key]).length;
          } catch {
            // не измерилось — не мешает удалению
          }
          delete tools[key];
          removed += 1;
        }
        if (removed === 0) return;

        const stats = await readB2Stats();
        stats.removed += removed;
        stats.bytesTrimmed += bytesTrimmed;
        const per = stats.byAgent[agent] ?? { removed: 0, bytesTrimmed: 0 };
        per.removed += removed;
        per.bytesTrimmed += bytesTrimmed;
        stats.byAgent[agent] = per;
        await ctx.storage.set(B2_STATS_KEY, stats);
        console.log(
          `[token-guard] B2 agent=${agent} removed=${removed} bytes=${bytesTrimmed}`,
        );
      } catch (e) {
        // Схемы/права при ошибке не трогаем: событие остаётся как есть.
        console.warn("[token-guard] B2 context failed, tools untouched:", e);
      }
    });
  },
});
