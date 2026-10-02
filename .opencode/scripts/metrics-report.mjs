#!/usr/bin/env node
// T-15 · B0-own P4 — metrics-report.mjs: отчёты метрик из нативных источников.
// Перенос прототипа BO-i4 из temp-полигона (B0-own; отчёт wave0b-own-report.md).
// Не канон; служебная зона. CLI-only (без БД): opencode stats --json ·
// session list · session export. Сводка проекта + свёртка цепочки
// root→субагенты по ролям/моделям (замена Opencode Telemetry для D; вход C6/D).
// Запуск: node .opencode/scripts/metrics-report.mjs [--chain <ses_root>] [--out <file.md>]
// Примечание: у temp-проектов projectID = "global" — список фильтруется по cwd.
// Грабли: `opencode` на Windows — npm-шим (.ps1/.cmd), нужен shell: true (DEP0190).

import { execFileSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";

const args = process.argv.slice(2);
const opt = (n, d = null) => {
  const i = args.indexOf(n);
  return i >= 0 ? args[i + 1] : d;
};
const chain = opt("--chain");
const out = opt("--out");

const run = (cmd, a) =>
  execFileSync(cmd, a, {
    encoding: "utf8",
    maxBuffer: 128 * 1024 * 1024,
    shell: process.platform === "win32",
  });

const fmt = (n) =>
  n == null ? "—" : typeof n === "number" ? n.toLocaleString("ru-RU") : String(n);
const usd = (n) => (n == null ? "—" : "$" + Number(n).toFixed(4));
const tok = (t) =>
  t == null
    ? "—"
    : typeof t === "object"
      ? Object.entries(t)
          .map(
            ([k, v]) =>
              `${k}=${typeof v === "number" ? v.toLocaleString("ru-RU") : JSON.stringify(v)}`,
          )
          .join(" · ")
      : String(t);
const iso = (t) => (t ? new Date(t).toISOString() : "—");
const mins = (a, b) => (a && b ? ((b - a) / 60000).toFixed(1) : "—");

const lines = [];
const push = (s = "") => {
  lines.push(s);
  console.log(s);
};

// --- stats
let stats = null;
try {
  stats = JSON.parse(run("opencode", ["stats", "--json"]));
} catch (e) {
  console.error("[stats] " + e.message);
}

// --- sessions list (фильтр по cwd)
let list = [];
try {
  list = JSON.parse(run("opencode", ["session", "list", "--format", "json", "-n", "200"]));
} catch (e) {
  console.error("[list] " + e.message);
}
const cwd = process.cwd();
const local = list.filter((s) => s.directory === cwd);

push(`# metrics-report${chain ? ` — цепочка ${chain}` : ""}`);
push("");
push(`- Создан: ${new Date().toISOString()}`);
push(`- Каталог: ${cwd}`);
if (stats) {
  push(
    `- stats: sessions=${fmt(stats.sessions)} · subagents=${fmt(stats.subagents)} · prompts=${fmt(stats.prompts)} · steps=${fmt(stats.steps)} · tokens=${tok(stats.tokens)} · cost=${usd(stats.cost)}`,
  );
}
push(`- session list: ${list.length} (в каталоге: ${local.length})`);

// --- chain
const exportSession = (id) => JSON.parse(run("opencode", ["session", "export", id]));
const childIds = (obj) => {
  const found = new Set();
  const walk = (x) => {
    if (typeof x === "string") {
      const re = /<subagent sessionID="(ses_[^"]+)"/g;
      let m;
      while ((m = re.exec(x))) found.add(m[1]);
    } else if (Array.isArray(x)) x.forEach(walk);
    else if (x && typeof x === "object") Object.values(x).forEach(walk);
  };
  walk(obj);
  return [...found];
};
const rowOf = (j) => {
  const info = j.info ?? {};
  return {
    id: info.id,
    agent: info.agent ?? null,
    model: info.model ?? null,
    parentID: info.parentID ?? null,
    outcome: info.outcome ?? null,
    cost: info.cost ?? null,
    tokens: info.tokens ?? null,
    created: info.time?.created ?? null,
    updated: info.time?.updated ?? null,
  };
};

if (chain) {
  const rootJson = exportSession(chain);
  const root = rowOf(rootJson);
  const kids = childIds(rootJson);
  const rows = [root];
  for (const k of kids) {
    try {
      rows.push(rowOf(exportSession(k)));
    } catch (e) {
      push(`- child ${k}: export error: ${e.message}`);
    }
  }
  const byAgent = new Map();
  const byModel = new Map();
  let cost = 0;
  let tin = 0;
  let tout = 0;
  let last = 0;
  for (const r of rows) {
    cost += r.cost ?? 0;
    tin += r.tokens?.input ?? 0;
    tout += r.tokens?.output ?? 0;
    if (r.updated && r.updated > last) last = r.updated;
    const a = r.agent ?? "(root)";
    const ba = byAgent.get(a) ?? { n: 0, cost: 0 };
    ba.n++;
    ba.cost += r.cost ?? 0;
    byAgent.set(a, ba);
    const m = r.model
      ? `${r.model.providerID}/${r.model.id}${r.model.variant ? "#" + r.model.variant : ""}`
      : "(?)";
    const bm = byModel.get(m) ?? { n: 0, cost: 0 };
    bm.n++;
    bm.cost += r.cost ?? 0;
    byModel.set(m, bm);
  }
  push("");
  push(`## Цепочка: ${rows.length} сессий (root + ${kids.length} детей)`);
  push(`- Стоимость: ${usd(cost)} · вх. ${fmt(tin)} · вых. ${fmt(tout)}`);
  push(`- Окно: ${iso(root.created)} → ${iso(last)} (${mins(root.created, last)} мин)`);
  push("");
  push("| роль | сессий | стоимость |");
  push("|---|---|---|");
  for (const [a, v] of [...byAgent].sort((x, y) => y[1].cost - x[1].cost))
    push(`| ${a} | ${v.n} | ${usd(v.cost)} |`);
  push("");
  push("| модель | сессий | стоимость |");
  push("|---|---|---|");
  for (const [m, v] of [...byModel].sort((x, y) => y[1].cost - x[1].cost))
    push(`| ${m} | ${v.n} | ${usd(v.cost)} |`);
  push("");
  push("| сессия | роль | модель | исход | стоимость | вх. | вых. |");
  push("|---|---|---|---|---|---|---|");
  for (const r of rows)
    push(
      `| ${r.id} | ${r.agent ?? "—"} | ${r.model ? r.model.id + (r.model.variant ? "#" + r.model.variant : "") : "—"} | ${r.outcome ?? "—"} | ${usd(r.cost)} | ${fmt(r.tokens?.input)} | ${fmt(r.tokens?.output)} |`,
    );
}

if (out) {
  mkdirSync(dirname(out), { recursive: true });
  writeFileSync(out, lines.join("\n") + "\n");
  console.log(`\n[written] ${out}`);
}
