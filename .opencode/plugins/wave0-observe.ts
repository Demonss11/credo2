// T-15 · B0-own P1 — wave0-observe: наблюдаемость субагентов.
// Перенос прототипа BO-i3 из temp-полигона (B0-own; отчёт wave0b-own-report.md).
// Не канон; служебная зона. Что делает: поток событий сессий (родитель/ребёнок)
// → bounded-журнал + агрегат; сводка родителю — только при WAVE0_OBSERVE_SUMMARY=1.
// Пишет (в gitignore): target/wave0-observe.jsonl (ротация в .1 при > 8 МБ),
//                      target/wave0-observe-summary.json.
// Фильтр: события location == каталог репозитория; далее — только известные
//         sessionID (поток серверный; у temp-проектов projectID = "global").

import { Plugin } from "@opencode/plugin";
import {
  appendFileSync,
  existsSync,
  renameSync,
  statSync,
  writeFileSync,
} from "node:fs";
import { join } from "node:path";

const MAX_BYTES = 8 * 1024 * 1024;

export default Plugin.define({
  id: "wave0-observe",
  async setup(ctx) {
    const dir = ctx.location.directory;
    const logPath = join(dir, "target", "wave0-observe.jsonl");
    const sumPath = join(dir, "target", "wave0-observe-summary.json");
    const summaryEnabled = process.env.WAVE0_OBSERVE_SUMMARY === "1";
    let appends = 0;

    const l = (data: Record<string, unknown>) => {
      try {
        if (++appends % 200 === 0 && existsSync(logPath)) {
          if (statSync(logPath).size > MAX_BYTES) renameSync(logPath, logPath + ".1");
        }
        appendFileSync(
          logPath,
          JSON.stringify({ t: new Date().toISOString(), ...data }) + "\n",
        );
      } catch {
        // журнал не должен ломать плагин
      }
    };

    const known = new Set<string>();
    const meta = new Map<string, any>();
    const timers = new Map<string, ReturnType<typeof setTimeout>>();
    const summarised = new Set<string>();

    const flush = () => {
      try {
        writeFileSync(
          sumPath,
          JSON.stringify(
            {
              at: new Date().toISOString(),
              summary: summaryEnabled,
              sessions: [...meta.entries()].map(([id, m]) => ({ id, ...m })),
            },
            null,
            2,
          ),
        );
      } catch {
        // агрегат не должен ломать плагин
      }
    };

    const scheduleSummary = (sid: string, parentID: string) => {
      const prev = timers.get(sid);
      if (prev) clearTimeout(prev);
      timers.set(
        sid,
        setTimeout(() => {
          timers.delete(sid);
          if (summarised.has(sid)) return;
          summarised.add(sid);
          if (!summaryEnabled) return;
          const m = meta.get(sid) ?? {};
          const text = `[wave0-observe] Сессия ${sid} (agent=${m.agent ?? "?"}) завершила активность: ${JSON.stringify(m.types ?? {})}; журнал — target/wave0-observe.jsonl`;
          void (async () => {
            try {
              await (ctx as any).session.prompt({ sessionID: parentID, text });
              l({ kind: "summary.prompt", shape: "text", sid, parentID, ok: true });
            } catch (e1: any) {
              try {
                await (ctx as any).session.prompt({
                  sessionID: parentID,
                  parts: [{ type: "text", text }],
                });
                l({ kind: "summary.prompt", shape: "parts", sid, parentID, ok: true });
              } catch (e2: any) {
                l({
                  kind: "summary.prompt",
                  shape: "both-failed",
                  sid,
                  parentID,
                  ok: false,
                  err1: String(e1?.message ?? e1),
                  err2: String(e2?.message ?? e2),
                });
              }
            }
          })();
        }, 5000),
      );
    };

    l({
      kind: "plugin.start",
      version: ctx.app.version,
      directory: dir,
      summary: summaryEnabled,
    });

    const controller = new AbortController();
    void (async () => {
      try {
        for await (const event of ctx.event.subscribe({ signal: controller.signal })) {
          const e: any = event;
          const loc = e?.location?.directory ?? null;
          const sid: string | null =
            e?.sessionID ??
            e?.data?.sessionID ??
            e?.data?.info?.sessionID ??
            e?.data?.session?.id ??
            e?.data?.info?.id ??
            null;
          const type = String(e?.type ?? "?");
          const ownLoc = typeof loc === "string" && loc === dir;
          if (!ownLoc && !(sid && known.has(sid))) continue;
          if (sid && (ownLoc || type === "session.created")) known.add(sid);
          const parentID: string | null =
            e?.data?.parentID ??
            e?.data?.info?.parentID ??
            e?.data?.session?.parentID ??
            null;
          const agent = e?.data?.agent ?? e?.data?.info?.agent ?? null;
          const model = e?.data?.model ?? e?.data?.info?.model ?? null;
          if (sid && !meta.has(sid)) {
            meta.set(sid, {
              first: new Date().toISOString(),
              parentID,
              agent,
              model,
              types: {},
              tools: {},
            });
          }
          if (sid) {
            const m = meta.get(sid);
            m.last = new Date().toISOString();
            m.types[type] = (m.types[type] || 0) + 1;
            const tool = e?.data?.tool ?? e?.data?.name ?? null;
            if (tool) m.tools[tool] = (m.tools[tool] || 0) + 1;
            flush();
            if (parentID && parentID !== sid) scheduleSummary(sid, parentID);
          }
          l({
            kind: "event",
            type,
            sid,
            parentID,
            agent,
            model,
            preview: JSON.stringify(e?.data ?? null).slice(0, 400),
          });
        }
      } catch (e: any) {
        if (!controller.signal.aborted) {
          l({ kind: "stream.error", message: String(e?.message ?? e) });
        }
      }
    })();

    return () => {
      controller.abort();
      for (const t of timers.values()) clearTimeout(t);
      flush();
    };
  },
});
