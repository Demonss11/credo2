// T-15 · C10 (B0-own P2) — wave0-guard: контекстные страховки + аудит.
// Перенос прототипа BO-i2 из temp-полигона (отчёт wave0b-own-report.md).
// Не канон; служебная зона. Слой поверх статических `experimental.policies`
// (глобальный конфиг): контекстные правила + аудит решений.
// Пишет (в gitignore): target/wave0-guard.jsonl (ротация в .1 при > 8 МБ).
// Правила: shell — `--force`, `reset --hard`, `Remove-Item -Recurse -Force`;
//          read — `*.env` (кроме `.env.example`);
//          edit — абсолютный путь вне location.

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
  id: "wave0-guard",
  async setup(ctx) {
    const dir = ctx.location.directory;
    const logPath = join(dir, "target", "wave0-guard.jsonl");
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
        // аудит не должен ломать плагин
      }
    };

    l({ kind: "plugin.start", version: ctx.app.version, directory: dir });

    await ctx.permission.hook("evaluate", (e: any) => {
      try {
        const res: string[] = Array.isArray(e.resources)
          ? e.resources.map((x: unknown) => String(x))
          : [String(e.resources ?? "")];
        const joined = res.join(" | ");
        let why: string | null = null;
        if (e.action === "shell") {
          // Якорные правила: блокируем ДЕЙСТВИЕ, а не упоминание подстроки
          // (широкие `*--force*`/`*.env*` ловят текст команд: echo, rg, commit -m).
          for (const r of res) {
            const c = String(r).trim();
            if (/^git\s+push\b[\s\S]*--force\b/.test(c)) {
              why = "git-push-force";
              break;
            }
            if (/^git\s+reset\s+--hard\b/.test(c)) {
              why = "git-reset-hard";
              break;
            }
            if (/^git\s+push\b/.test(c) && /\s-f(\s|$)/.test(c)) {
              why = "git-push-force-short";
              break;
            }
            if (
              /^(Get-Content|gc|cat|type|more|less|head|tail|Select-String|rg|grep)\b/i.test(c) &&
              /(^|[\\/\s'"])\.env(\.[A-Za-z0-9_-]+)?(['"\s]|$)/i.test(c) &&
              !/\.env\.example/i.test(c)
            ) {
              why = "secrets-env";
              break;
            }
            if (
              /^(Get-Content|gc|cat|type|more|less|head|tail|Select-String|rg|grep)\b/i.test(c) &&
              /[\\/]\.ssh[\\/]/i.test(c)
            ) {
              why = "ssh-read";
              break;
            }
            if (/^Remove-Item\b/i.test(c) && /-Recurse/i.test(c) && /-Force/i.test(c)) {
              why = "recursive-force-delete";
              break;
            }
          }
        } else if (e.action === "read") {
          if (
            /[\\/]?\.env(\..+)?$/i.test(joined) &&
            !/\.env\.example$/i.test(joined)
          ) {
            why = "secrets-env";
          }
        } else if (e.action === "edit") {
          // Внешние пути — только аудит: default `external_directory` (ask) уже
          // покрывает доступ; deny здесь ломает служебные операции владельца
          // (temp-полигоны, глобальный конфиг) — факт P13.
          const first = res[0] ?? "";
          if (
            /^(?:[A-Za-z]:[\\/]|\\\\|\/)/.test(first) &&
            !first.toLowerCase().startsWith(dir.toLowerCase())
          ) {
            l({ kind: "outside-edit", action: e.action, agent: e.agent ?? null, resources: res });
          }
        }
        if (why) {
          e.effect = "deny";
          e.message = `wave0-guard: ${why}`;
          l({ kind: "deny", action: e.action, agent: e.agent ?? null, resources: res, why });
          return;
        }
        l({
          kind: "evaluate",
          action: e.action,
          agent: e.agent ?? null,
          resources: res,
          effect: e.effect,
        });
      } catch (err: any) {
        l({ kind: "error", message: String(err?.message ?? err) });
      }
    });
  },
});
