#!/usr/bin/env node
// T-15 · C6 (D96) — state-metrics.mjs: метрики процесса и очереди ИЗ СОСТОЯНИЯ.
// Не канон; служебная зона. Query-слой из .opencode/state/current/*.yaml
// (progress, receipts, current_state, next_action). Без нативных метрик сессий
// (стоимость/токены) — они у metrics-report.mjs (B0-own P4).
//
// Запуск: node .opencode/scripts/state-metrics.mjs [--json] [--dir <state/current>]
//         [--task T-XX] [--session N] [--out <file.md>]
// Секции (D96 п.3): прогон (задачи/сессии/шаги/упоры) · dispatch по ролям ·
// re-plan по категориям + конвергенция (без owner_override) · очереди
// deferred_by_owner/blocked · квитанции (iteration/verdict).
//
// Мини-YAML-парсер без зависимостей: читает списки записей `- key: value` с
// вложенными пары/массивы на 2+ пробелах отступа. Значения — скаляры (кавычки
// снимаются). Многострочные блоки (`|`/`>`) схлопываются в строку.

import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

const args = process.argv.slice(2);
const has = (n) => args.includes(n);
const opt = (n, d = null) => {
  const i = args.indexOf(n);
  return i >= 0 ? args[i + 1] : d;
};

const root = join(process.cwd(), ".opencode", "state");
const dir = opt("--dir") ?? join(root, "current");
const taskFilter = opt("--task");
const sessionFilter = opt("--session") == null ? null : Number(opt("--session"));
const asJson = has("--json");
const out = opt("--out");

// ── мини-YAML ───────────────────────────────────────────────────────────────
const scalar = (s) => {
  let v = s.trim();
  const q = v.match(/^"(.*)"$/) ?? v.match(/^'(.*)'$/);
  if (q) return q[1].replace(/\\"/g, '"');
  if (v === "true") return true;
  if (v === "false") return false;
  if (/^-?\d+$/.test(v)) return Number(v);
  return v;
};
const readYaml = (path) => {
  let text = "";
  try {
    text = readFileSync(path, "utf8");
  } catch {
    return null;
  }
  const lines = text.split(/\r?\n/);
  // Одиночный мап vs список записей: список — только `- ` с нулевым отступом
  // (вложенные списки в мапах — не счёт).
  const hasList = lines.some((l) => /^- /.test(l));
  if (!hasList) {
    // Полный вложенный парсер мапа: ключи, вложенные мапы (2 пробела), списки.
    const root = {};
    const stack = [{ indent: -1, node: root }];
    for (let i = 0; i < lines.length; i++) {
      const raw = lines[i];
      if (!raw.trim() || raw.trimStart().startsWith("#")) continue;
      const indent = raw.length - raw.trimStart().length;
      const line = raw.trim();
      while (stack.length > 1 && indent <= stack[stack.length - 1].indent)
        stack.pop();
      const parent = stack[stack.length - 1].node;
      if (line.startsWith("- ")) {
        if (!Array.isArray(parent)) continue;
        parent.push(scalar(line.slice(2)));
        continue;
      }
      const m = line.match(/^([\w.-]+):\s*(.*)$/);
      if (!m) continue;
      const [, key, val] = m;
      if (val === "") {
        // Не знаем заранее: мап или список — смотрим следующую значимую строку.
        let next = null;
        for (let j = i + 1; j < lines.length; j++) {
          if (lines[j].trim() && !lines[j].trimStart().startsWith("#")) {
            next = lines[j];
            break;
          }
        }
        const isList =
          next != null &&
          /^\s*- /.test(next) &&
          next.length - next.trimStart().length > indent;
        const node = isList ? [] : {};
        parent[key] = node;
        stack.push({ indent, node });
      } else {
        parent[key] = scalar(val);
      }
    }
    return [root];
  }
  const records = [];
  let cur = null;
  let pendingKey = null;
  for (const raw of lines) {
    if (!raw.trim() || raw.trimStart().startsWith("#")) continue;
    const indent = raw.length - raw.trimStart().length;
    const line = raw.trim();
    if (/^- /.test(raw)) {
      cur = {};
      records.push(cur);
      pendingKey = null;
      const rest = line.slice(2);
      const m = rest.match(/^([\w.-]+):\s*(.*)$/);
      if (m) cur[m[1]] = m[2] === "" ? null : scalar(m[2]);
      continue;
    }
    if (!cur) continue;
    const m = line.match(/^([\w.-]+):\s*(.*)$/);
    if (!m) continue;
    const [, key, val] = m;
    if (val === "" || val === "|" || val === ">") {
      cur[key] = val === "" ? {} : "";
      pendingKey = key;
    } else {
      cur[key] = scalar(val);
      pendingKey = null;
    }
    void indent;
    void pendingKey;
  }
  return records;
};

const progress = readYaml(join(dir, "progress.yaml")) ?? [];
const receipts = readYaml(join(dir, "receipts.yaml")) ?? [];
const state = (readYaml(join(dir, "current_state.yaml")) ?? [])[0] ?? {};
const plan = readYaml(join(dir, "next_action.yaml")) ?? [];
const plan0 = plan[0] ?? {};

const keepProgress = (r) =>
  (!taskFilter || r.task === taskFilter) &&
  (sessionFilter == null || r.session_index === sessionFilter);
// Квитанции: `session_index` в схеме нет (state-schema.md §receipts) — фильтр
// только по задаче.
const keepReceipt = (r) => !taskFilter || r.task === taskFilter;
const P = progress.filter(keepProgress);
const R = receipts.filter(keepReceipt);

// ── агрегаты ────────────────────────────────────────────────────────────────
const grid = (rows, key, value) => {
  const m = new Map();
  for (const r of rows) {
    const k = r[key];
    if (k == null) continue;
    m.set(k, (m.get(k) ?? 0) + (typeof value === "function" ? value(r) : 1));
  }
  return m;
};
const sorted = (m) => [...m].sort((a, b) => b[1] - a[1]);
const fmt = (n) => (n == null ? "—" : String(n));
const uniq = (arr) => [...new Set(arr.filter((v) => v != null && v !== ""))];

const tasks = uniq(P.map((r) => r.task));
const sessions = uniq(P.map((r) => r.session_index)).sort((a, b) => a - b);
const roles = grid(P.filter((r) => r.action === "dispatch"), "role");
const stepsBySession = new Map();
const readByIdx = new Map();
for (const r of P) {
  if (r.session_index == null) continue;
  stepsBySession.set(r.session_index, (stepsBySession.get(r.session_index) ?? 0) + 1);
  // Упор лимита: маркер в result/note (в схеме отдельного поля нет).
  if (/упор|лимит(?:а|ом)?\s*(?:шагов|steps)?|steps=|reach(?:ed)? (?:the )?limit/i.test(
    `${r.result ?? ""} ${r.note ?? ""}`,
  ))
    readByIdx.set(r.session_index, (readByIdx.get(r.session_index) ?? 0) + 1);
}

// re-plan по категориям: категория — из записи (`replan_reason` — опционально,
// `state-schema.md`); при отсутствии — «(не указана)». Зеркало текущего
// состояния (`re_raise`/`blocker`) — отдельной строкой, не подменяет категории
// записей (D90).
const replan = new Map();
let replanTotal = 0;
let convergence = 0;
for (const r of P) {
  if (r.action !== "re-plan") continue;
  replanTotal += 1;
  const cat = r.replan_reason ?? "(не указана)";
  replan.set(cat, (replan.get(cat) ?? 0) + 1);
  if (cat !== "owner_override") convergence += 1;
}
const stateCategory = state.re_raise?.category ?? state.blocker?.category ?? "—";

// очереди: только нерешённые (resolved !== true)
const deferred = [];
const blocked = [];
const walkRe = (obj, key) => {
  if (!obj || typeof obj !== "object") return;
  for (const [k, v] of Object.entries(obj)) {
    const isDeferred = /deferred/i.test(k);
    const isBlocked = k === "blocker" || k === "blocked";
    if ((isDeferred || isBlocked) && v && typeof v === "object" && !Array.isArray(v)) {
      if (v.resolved !== true) (isBlocked ? blocked : deferred).push(v);
      continue;
    }
    if (Array.isArray(v)) {
      for (const item of v) {
        if (isDeferred && item && typeof item === "object" && item.resolved !== true)
          deferred.push(item);
        else if (isDeferred && typeof item === "string") deferred.push(item);
        else walkRe(item, key);
      }
      continue;
    }
    walkRe(v, k);
  }
};
walkRe(state);
walkRe(plan0);

// квитанции
const openTask = state.task ?? plan0.task ?? "(нет)";

// ── вывод ───────────────────────────────────────────────────────────────────
const L = [];
const push = (s = "") => L.push(s);

push("# state-metrics — метрики процесса из состояния");
push("");
push(`- Создан: ${new Date().toISOString()}`);
push(`- Источник: \`${dir}\` (progress/receipts/current_state/next_action)`);
push(`- Текущая задача: ${fmt(openTask)} · сессия: ${fmt(state.session_index)}`);
push(`- Фильтр: task=${taskFilter ?? "—"} · session=${sessionFilter ?? "—"}`);
push("");

push("## Прогон");
push(
  `- Записей progress: ${P.length} · задач: ${tasks.length} (${tasks.join(", ") || "—"})`,
);
push(`- Сессий: ${sessions.length} (${sessions.join(", ") || "—"})`);
push("");
push("| сессия | шагов | упоров лимита |");
push("|---|---|---|");
for (const [s, n] of [...stepsBySession].sort((a, b) => a[0] - b[0]))
  push(`| s${s} | ${n} | ${readByIdx.get(s) ?? 0} |`);
push("");

push("## Dispatch по ролям");
push("| роль | вызовов |");
push("|---|---|");
for (const [r, n] of sorted(roles)) push(`| ${r} | ${n} |`);
if (roles.size === 0) push("| — | 0 |");
push("");

push("## Re-plan: категории и конвергенция");
push(
  `- Всего re-plan: ${replanTotal} · метрика конвергенции (без owner_override): ${convergence}`,
);
push(`- Категория текущего состояния (re_raise/blocker): ${stateCategory}`);
push("");
push("| категория | re-plan |");
push("|---|---|");
for (const [c, n] of sorted(replan)) push(`| ${c} | ${n} |`);
if (replan.size === 0) push("| — | 0 |");
push("");

push("## Очереди");
push(`- deferred_by_owner: ${deferred.length}`);
for (const d of deferred.slice(0, 20))
  push(`  - ${typeof d === "string" ? d : JSON.stringify(d)}`);
push(`- blocked: ${blocked.length}`);
for (const b of blocked.slice(0, 20))
  push(`  - ${typeof b === "string" ? b : JSON.stringify(b)}`);
push("");

push("## Квитанции");
push("| verdict | iteration | записей |");
push("|---|---|---|");
const byVerdictIter = new Map();
for (const r of R) {
  const k = `${r.verdict ?? "—"}\u0000${r.iteration ?? "—"}`;
  byVerdictIter.set(k, (byVerdictIter.get(k) ?? 0) + 1);
}
for (const [k, n] of [...byVerdictIter].sort()) {
  const [v, it] = k.split("\u0000");
  push(`| ${v} | ${it} | ${n} |`);
}
if (byVerdictIter.size === 0) push("| — | — | 0 |");
push("");

const report = L.join("\n");
console.log(report);

const json = {
  generated_at: new Date().toISOString(),
  source_dir: dir,
  filter: { task: taskFilter, session: sessionFilter },
  open: { task: openTask, session_index: state.session_index ?? null },
  run: {
    progress_records: P.length,
    tasks,
    sessions,
    steps_by_session: Object.fromEntries([...stepsBySession]),
    limit_hits_by_session: Object.fromEntries([...readByIdx]),
  },
  dispatch_by_role: Object.fromEntries(sorted(roles)),
  replan: {
    total: replanTotal,
    convergence: convergence,
    by_category: Object.fromEntries(sorted(replan)),
    state_category: stateCategory,
  },
  queues: {
    deferred_by_owner: deferred.length,
    blocked: blocked.length,
    deferred,
    blocked_items: blocked,
  },
  receipts: {
    by_verdict_iteration: Object.fromEntries(
      [...byVerdictIter].map(([k, n]) => [k.replace("\u0000", " / iter "), n]),
    ),
  },
};

if (asJson) console.log("\n" + JSON.stringify(json, null, 2));
if (out) {
  mkdirSync(dirname(out), { recursive: true });
  writeFileSync(out, report + "\n");
  console.log(`\n[written] ${out}`);
}
