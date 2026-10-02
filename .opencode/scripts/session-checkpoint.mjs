#!/usr/bin/env node
// T-15 · B0-own P3 — session-checkpoint.mjs: структурная сводка останова/обрыва.
// Перенос прототипа BO-i5 из temp-полигона (B0-own; отчёт wave0b-own-report.md).
// Не канон; служебная зона. CLI-only: `opencode session export <id>`.
// Запуск: node .opencode/scripts/session-checkpoint.mjs --session <ses_id> [--out <file.md>]
// Сводка: цель · статус · файлы (изменённые/прочитанные) · маркеры рисков ·
// метрики · resume-путь. Доработка относительно прототипа: файлы разделены на
// изменённые (edit/write/patch, snapshot) и прочитанные (read).

import { execFileSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";

const args = process.argv.slice(2);
const opt = (n, d = null) => {
  const i = args.indexOf(n);
  return i >= 0 ? args[i + 1] : d;
};
const ses = opt("--session", args.find((a) => a.startsWith("ses_")) ?? null);
const out = opt("--out");
if (!ses) {
  console.error(
    "usage: node .opencode/scripts/session-checkpoint.mjs --session <ses_id> [--out <file.md>]",
  );
  process.exit(2);
}

const run = (cmd, a) =>
  execFileSync(cmd, a, {
    encoding: "utf8",
    maxBuffer: 128 * 1024 * 1024,
    shell: process.platform === "win32",
  });

const j = JSON.parse(run("opencode", ["session", "export", ses]));
const info = j.info ?? {};
const msgs = j.messages ?? [];

const unquote = (s) => String(s ?? "").replace(/^"|"$/g, "");
const users = msgs.filter((m) => m.type === "user");
const asst = msgs.filter((m) => m.type === "assistant");
const textOf = (m) =>
  (m.content ?? [])
    .filter((c) => c.type === "text")
    .map((c) => c.text)
    .join("\n");

const CHANGE_TOOLS = new Set(["edit", "write", "patch"]);
const tools = {};
const changed = new Set();
const readFiles = new Set();
for (const m of asst) {
  for (const c of m.content ?? []) {
    if (c.type === "tool") {
      tools[c.name] = (tools[c.name] ?? 0) + 1;
      const inp = c.state?.input ?? {};
      const path = ["filePath", "path", "file"].map((k) => inp[k]).find((v) => typeof v === "string");
      if (path) (CHANGE_TOOLS.has(c.name) ? changed : readFiles).add(path);
    }
  }
  for (const f of m.snapshot?.files ?? []) changed.add(f);
}

const re = /(открыт|риск|блокер|осталось|не сделано|вопрос|TODO|хвост)/i;
const markers = [];
for (const m of asst) {
  for (const line of textOf(m).split(/\r?\n/)) {
    if (re.test(line)) markers.push(line.trim());
  }
}

const last = asst.length ? textOf(asst[asst.length - 1]).trim() : "";
const goal = users.length ? unquote(users[0].text) : "";
const model = info.model
  ? `${info.model.providerID}/${info.model.id}${info.model.variant ? "#" + info.model.variant : ""}`
  : "—";
const end = info.time?.idle ?? info.time?.updated;
const wall =
  info.time?.created && end ? ((end - info.time.created) / 60000).toFixed(1) : "—";

const lines = [];
const push = (s = "") => {
  lines.push(s);
  console.log(s);
};
push(`# Checkpoint: ${ses}`);
push("");
push(`- Создан: ${new Date().toISOString()}`);
push(`- Заголовок: ${info.title ?? "—"}`);
push(`- Агент/модель: ${info.agent ?? "(root)"} · ${model}`);
push(
  `- Исход: ${info.outcome ?? "—"} · стоимость: $${(info.cost ?? 0).toFixed(4)} · окно: ${wall} мин`,
);
push(
  `- Сообщений: ${msgs.length} · ассистентских: ${asst.length} · инструменты: ${Object.entries(tools).map(([k, v]) => `${k}×${v}`).join(", ") || "—"}`,
);
push("");
push(`## Цель (первый промпт)`);
push(goal ? `> ${goal.slice(0, 600)}` : "—");
push("");
push(`## Статус (последний текст)`);
push(last ? last.slice(0, 1200) : "—");
push("");
push(`## Файлы изменённые (${changed.size})`);
for (const f of [...changed].slice(0, 40)) push(`- ${f}`);
push("");
push(`## Файлы прочитанные (${readFiles.size})`);
for (const f of [...readFiles].slice(0, 40)) push(`- ${f}`);
push("");
push(`## Риски/открытые вопросы (${markers.length})`);
for (const l of markers.slice(0, 20)) push(`- ${l.slice(0, 300)}`);
push("");
push(`## Продолжение`);
push(
  `- Resume: \`opencode run --session ${ses} --auto --model ${info.model ? info.model.providerID + "/" + info.model.id : "<model>"} "продолжи"\``,
);
push(`- Альтернативы: \`--continue\` (последняя сессия), \`--fork\` (ветка от сессии)`);

if (out) {
  mkdirSync(dirname(out), { recursive: true });
  writeFileSync(out, lines.join("\n") + "\n");
  console.log(`\n[written] ${out}`);
}
