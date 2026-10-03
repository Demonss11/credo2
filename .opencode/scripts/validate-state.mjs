#!/usr/bin/env node
// validate-state.mjs — валидатор схемы состояния процесса (T-15, C2; D91).
//
// Назначение: машинная проверка состояния (.opencode/state/current/*.yaml) по
// контракту `.opencode/rules/state-schema.md` (канон, D86). Точки применения —
// pre-flight прогона и приёмка `validator` (адресный прогон).
//
// Контракт:
//   - обязательные поля артефактов, enum'ы, формат дат (YYYY-MM-DD);
//   - инварианты: iteration ≥ 1; rework = iteration − 1; единое `iteration` в
//     плане/состоянии/progress/receipts текущего раунда; session_index не
//     убывает; при action: surface_to_user — channel и owner_response;
//     result/next не пустые;
//   - записи progress/receipts, созданные до принятия схемы, — мягко
//     (недостающие поля — предупреждение, не ошибка);
//   - неизвестные поля — предупреждение, не ошибка (эволюция без жёсткости).
//
// Использование:
//   node .opencode/scripts/validate-state.mjs [--dir <path>] [--file <artifact>]
//        [--json] [--strict] [--since YYYY-MM-DD]
//   --dir    каталог состояния (по умолчанию .opencode/state/current)
//   --file   один артефакт: next_action | current_state | progress | receipts
//   --json   машиночитаемый отчёт
//   --strict неизвестные поля считать ошибками
//   --since  дата начала машинной проверки (по умолчанию 2026-10-03); записи
//            раньше — мягко (недостающие поля — предупреждение)
//
// Коды выхода: 0 — ошибок нет; 1 — есть ошибки схемы; 2 — ошибка запуска/чтения.
//
// Флаги `--json`/`--strict` — контракт скрипта; `state-schema.md` ссылается на
// него как на точку машинной проверки. Самодостаточный (без внешних
// зависимостей): парсится подмножество YAML, достаточное для артефактов
// состояния (блочные map/list, скаляры, блочные скаляры >/|-, вложенные
// структуры, комментарии).

import { readFileSync, existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const REPO_ROOT = resolve(here, "..", "..");

// --- CLI ---------------------------------------------------------------

const argv = process.argv.slice(2);
const opt = (name, def = null) => {
  const i = argv.indexOf(name);
  return i >= 0 && i + 1 < argv.length ? argv[i + 1] : def;
};

const dirArg = opt("--dir");
const fileArg = opt("--file");
const asJson = argv.includes("--json");
const strict = argv.includes("--strict");

const ARTIFACTS = ["next_action", "current_state", "progress", "receipts"];

if (fileArg && !ARTIFACTS.includes(fileArg)) {
  console.error(`validate-state: неизвестный артефакт «${fileArg}». Ожидалось: ${ARTIFACTS.join(", ")}`);
  process.exit(2);
}

const stateDir = dirArg
  ? resolve(process.cwd(), dirArg)
  : join(REPO_ROOT, ".opencode", "state", "current");

if (!existsSync(stateDir)) {
  console.error(`validate-state: каталог состояния не найден: ${stateDir}`);
  process.exit(2);
}

const files = fileArg ? [fileArg] : ARTIFACTS;

// --- Мини-парсер YAML (подмножество) -----------------------------------

const indentOf = (line) => line.length - line.trimStart().length;

// Убирает inline-комментарий (# …), не трогая # внутри кавычек.
function stripInlineComment(text) {
  let quote = null;
  for (let i = 0; i < text.length; i += 1) {
    const ch = text[i];
    if (quote) {
      if (ch === quote) quote = null;
    } else if (ch === '"' || ch === "'") {
      quote = ch;
    } else if (ch === "#" && (i === 0 || /\s/.test(text[i - 1]))) {
      return text.slice(0, i);
    }
  }
  return text;
}

function parseScalar(text) {
  const s = text.trim();
  if (s === "") return null;
  if (s === "null" || s === "~") return null;
  if (s === "true") return true;
  if (s === "false") return false;
  if (/^-?\d+$/.test(s)) return Number(s);
  if (
    (s.startsWith('"') && s.endsWith('"') && s.length >= 2) ||
    (s.startsWith("'") && s.endsWith("'") && s.length >= 2)
  ) {
    return s.slice(1, -1);
  }
  return s;
}

// Разбирает содержимое по строкам в дерево; каждый узел несёт `__line`
// (номер строки в файле) для читаемых сообщений. Достаточно для state-файлов.
function parseYaml(raw) {
  const lines = raw.replace(/^\uFEFF/, "").split(/\r?\n/);
  let i = 0;

  const isComment = (line) => line.trim() === "" || line.trimStart().startsWith("#");

  const readBlockScalar = (baseIndent) => {
    const parts = [];
    i += 1;
    while (i < lines.length) {
      const line = lines[i];
      if (line.trim() === "") {
        parts.push("");
        i += 1;
        continue;
      }
      if (indentOf(line) <= baseIndent) break;
      parts.push(line.slice(baseIndent + 1).trimEnd());
      i += 1;
    }
    while (parts.length && parts[parts.length - 1] === "") parts.pop();
    return parts.join("\n");
  };

  const parseBlock = (minIndent) => {
    const skipEmpty = () => {
      while (i < lines.length && isComment(lines[i])) i += 1;
    };

    skipEmpty();
    if (i >= lines.length) return null;
    const indent = indentOf(lines[i]);
    if (indent < minIndent) return null;
    const startLine = i + 1;

    const wrap = (value) => {
      if (value && typeof value === "object" && !Array.isArray(value)) {
        Object.defineProperty(value, "__line", { value: startLine, enumerable: false });
      }
      return value;
    };

    // список?
    if (/^\s*-\s/.test(lines[i]) || /^\s*-\s*$/.test(lines[i])) {
      const arr = [];
      while (i < lines.length) {
        skipEmpty();
        if (i >= lines.length) break;
        const li = lines[i];
        if (indentOf(li) !== indent || !/^\s*-(\s|$)/.test(li)) break;
        const itemLine = i + 1;
        const rest = li.replace(/^\s*-\s?/, "");
        if (rest === "") {
          i += 1;
          const child = parseBlock(indent + 1);
          if (child && typeof child === "object" && !Array.isArray(child))
            Object.defineProperty(child, "__line", { value: itemLine, enumerable: false });
          arr.push(child);
          continue;
        }
        // элемент вида "- key: value"
        if (/^[^:]+:(.|\s|$)/.test(rest) && !rest.startsWith("{")) {
          const map = {};
          lines[i] = " ".repeat(indent + 2) + rest;
          const child = parseBlock(indent + 2);
          Object.assign(map, child);
          Object.defineProperty(map, "__line", { value: itemLine, enumerable: false });
          arr.push(map);
          continue;
        }
        arr.push(parseScalar(rest));
        i += 1;
      }
      return arr;
    }

    // map?
    const map = {};
    wrap(map);
    while (i < lines.length) {
      skipEmpty();
      if (i >= lines.length) break;
      const li = lines[i];
      if (indentOf(li) !== indent) {
        if (indentOf(li) < indent) break;
        throw new Error(`неожиданный отступ в строке ${i + 1}: ${li}`);
      }
      const m = /^([^:]+):(.*)$/.exec(li);
      if (!m) throw new Error(`ожидался «key: value» в строке ${i + 1}: ${li}`);
      const key = m[1].trim();
      const valueText = stripInlineComment(m[2]).trim();
      if (valueText === "") {
        i += 1;
        const child = parseBlock(indent + 1);
        map[key] = child;
      } else if (["|", "|-", ">", ">-", "|+", ">+"].includes(valueText)) {
        map[key] = readBlockScalar(indent);
      } else {
        map[key] = parseScalar(stripInlineComment(m[2]));
        i += 1;
      }
    }
    return map;
  };

  return parseBlock(0);
}

// --- Контракт схемы ----------------------------------------------------

const DATE_RE = /^\d{4}-\d{2}-\d{2}$/;

const REQUIRED = {
  next_action: {
    task: "string",
    iteration: "int",
    status: ["in_progress", "awaiting_user", "blocked", "idle"],
    progress_marker: "string",
    as_of: "date",
    next: "list",
    resume_hint: "string",
  },
  current_state: {
    as_of: "date",
    task: "string",
    iteration: "int",
    rework: "int",
    session_index: "int",
    phase: ["planning", "implementation", "verification", "closing", "done"],
    acceptance: ["pending", "rework", "accepted", "accepted_with_notes"],
  },
  progress: {
    at: "date",
    task: "string",
    iteration: "int",
    session_index: "int",
    action: ["dispatch", "surface_to_user", "wait_for_user", "re-plan", "complete"],
    result: "nonempty",
    next: "nonempty",
  },
  receipts: {
    task: "string",
    iteration: "int",
    verdict: ["accepted", "accepted_with_notes", "rework"],
    report: "string",
    at: "date",
  },
};

const OPTIONAL = {
  next_action: new Set([
    "task_kind", "class", "class_dispute", "class_note", "branch",
    "scope_constraints", "route_exclusions", "package", "re_raise",
    "inherited_boundaries",
  ]),
  current_state: new Set([
    "operation", "kind", "task_note", "class", "class_dispute", "status",
    "progress_marker", "acceptance_reports", "artifacts", "route_pending",
    "route_exclusions", "package", "base", "branch", "gate_pending",
    "worktree_remainder", "snapshot_check", "constraints", "resume",
    "mixed_worktree", "inherited_boundaries",
  ]),
  progress: new Set([
    "role", "expect_match", "channel", "owner_response", "question",
    "package", "pre_gate", "post_package", "round", "resume", "note",
    "replan_reason",
  ]),
  receipts: new Set(["dod", "snapshot"]),
};

// Мягкий режим для исторических записей: схема принята 02.10.2026 (D86), но
// машинная проверка введена с C2 (03.10.2026); записи раньше 03.10 — мягко
// (D91 п.5). Порог переопределяется `--since YYYY-MM-DD`.
const DEFAULT_SINCE = "2026-10-03";
const sinceArg = opt("--since");
const SINCE = /^\d{4}-\d{2}-\d{2}$/.test(sinceArg ?? "") ? sinceArg : DEFAULT_SINCE;
const SCHEMA_DATE = new Date(SINCE + "T00:00:00Z");
const isLegacy = (at) =>
  typeof at === "string" && DATE_RE.test(at) && new Date(at + "T00:00:00Z") < SCHEMA_DATE;

function checkType(value, spec) {
  if (Array.isArray(spec)) {
    return spec.includes(String(value))
      ? null
      : `значение «${value}» вне допустимых (${spec.join(" · ")})`;
  }
  switch (spec) {
    case "string":
      return typeof value === "string" && value.trim() !== "" ? null : "ожидалась непустая строка";
    case "nonempty":
      return typeof value === "string" && value.trim() !== "" ? null : "пустое значение";
    case "int":
      return Number.isInteger(value) ? null : "ожидалось целое число";
    case "bool":
      return typeof value === "boolean" ? null : "ожидалось true/false";
    case "date":
      return typeof value === "string" && DATE_RE.test(value) ? null : "ожидалась дата YYYY-MM-DD";
    case "list":
      return Array.isArray(value) && value.length > 0 ? null : "ожидался непустой список";
    default:
      return null;
  }
}

// --- Валидация ---------------------------------------------------------

const errors = [];
const warnings = [];
const err = (file, line, message) => errors.push({ file, line, message });
const warn = (file, line, message) => warnings.push({ file, line, message });

function recordsOf(doc, kind) {
  if (kind === "progress" || kind === "receipts") {
    if (doc == null) return [];
    if (Array.isArray(doc)) return doc;
    if (typeof doc === "object") {
      for (const k of ["entries", "actions", "items", "log", "records", "history"]) {
        if (Array.isArray(doc[k])) return doc[k];
      }
    }
    return null;
  }
  return doc;
}

function checkRecord(file, kind, rec, index) {
  const line = rec && typeof rec === "object" ? rec.__line ?? "?" : "?";
  if (rec == null || typeof rec !== "object" || Array.isArray(rec)) {
    err(file, line, `запись №${index} — не отображение (map)`);
    return;
  }
  const legacy = (kind === "progress" || kind === "receipts") && isLegacy(rec.at);
  const required = REQUIRED[kind];
  const optional = OPTIONAL[kind];

  const soft = (field, message) => {
    if (legacy) warn(file, line, `${field}: ${message} (историческая запись — мягкий режим)`);
    else err(file, line, `${field}: ${message}`);
  };

  for (const [field, spec] of Object.entries(required)) {
    if (!(field in rec) || rec[field] == null) {
      soft(field, `отсутствует обязательное поле «${field}»`);
      continue;
    }
    const problem = checkType(rec[field], spec);
    if (problem) soft(field, problem);
  }

  // условно обязательные / значения enum'ов
  if (kind === "progress") {
    if (rec.action === "surface_to_user") {
      if (!rec.channel) soft("channel", "action surface_to_user требует channel");
      else if (!["question", "text"].includes(rec.channel))
        soft("channel", `значение «${rec.channel}» вне допустимых (question · text)`);
      if (!rec.owner_response) soft("owner_response", "action surface_to_user требует owner_response (дословно)");
      if (!rec.question) soft("question", "action surface_to_user требует question");
    }
    if (rec.action === "dispatch") {
      if (!rec.role) soft("role", "action dispatch требует role");
      if (!rec.expect_match) soft("expect_match", "action dispatch требует expect_match");
      else if (!["true", "partial", "false"].includes(String(rec.expect_match)))
        warn(file, line, `expect_match: значение «${rec.expect_match}» вне допустимых (true · partial · false)`);
    }
    if (rec.action === "re-plan" && rec.replan_reason) {
      if (!["expect_mismatch", "owner_override", "plan_gap", "role_failure"].includes(rec.replan_reason))
        soft("replan_reason", `значение «${rec.replan_reason}» вне 4 категорий`);
    }
  }

  // неизвестные поля
  for (const field of Object.keys(rec)) {
    if (!(field in required) && !optional.has(field)) {
      const message = `неизвестное поле «${field}» — предупреждение (расширение схемы — журналом)`;
      if (strict) err(file, line, message);
      else warn(file, line, message);
    }
  }
}

// --- Инварианты --------------------------------------------------------

function checkNextAction(doc) {
  const file = "next_action.yaml";
  const line = doc.__line ?? "?";
  if (Number.isInteger(doc.iteration) && doc.iteration < 1)
    err(file, line, "iteration: iteration ≥ 1");
  if (Array.isArray(doc.next)) {
    const kinds = ["dispatch", "surface_to_user", "wait_for_user", "complete"];
    doc.next.forEach((action, idx) => {
      if (action == null || typeof action !== "object") {
        err(file, line, `next[${idx}]: действие — не отображение`);
        return;
      }
      if (!action.kind) err(file, action.__line ?? line, `next[${idx}].kind: отсутствует kind`);
      else if (!kinds.includes(action.kind))
        err(file, action.__line ?? line, `next[${idx}].kind: значение «${action.kind}» вне допустимых (${kinds.join(" · ")})`);
      if (action.kind === "dispatch" && !action.role)
        err(file, action.__line ?? line, `next[${idx}].role: dispatch требует role`);
    });
  }
}

function checkProgressInvariants(recs) {
  const file = "progress.yaml";
  let lastSession = 0;
  recs.forEach((rec, idx) => {
    if (rec == null || typeof rec !== "object") return;
    const line = rec.__line ?? "?";
    const legacy = isLegacy(rec.at);
    if (Number.isInteger(rec.iteration) && rec.iteration < 1)
      err(file, line, "iteration: iteration ≥ 1");
    if (Number.isInteger(rec.session_index)) {
      if (rec.session_index < 1) err(file, line, "session_index: session_index ≥ 1");
      if (rec.session_index < lastSession)
        err(file, line, `session_index: убывает (${lastSession} → ${rec.session_index})`);
      lastSession = Math.max(lastSession, rec.session_index);
    }
  });
}

function checkReceiptsInvariants(recs) {
  recs.forEach((rec) => {
    if (rec == null || typeof rec !== "object") return;
    if (Number.isInteger(rec.iteration) && rec.iteration < 1)
      err("receipts.yaml", rec.__line ?? "?", "iteration: iteration ≥ 1");
  });
}

// --- Кросс-инварианты (единое iteration, rework) -----------------------

function crossCheck() {
  const read = (kind) => {
    const path = join(stateDir, `${kind}.yaml`);
    if (!existsSync(path)) return null;
    try {
      return parseYaml(readFileSync(path, "utf-8"));
    } catch {
      return null;
    }
  };
  const nextAction = read("next_action");
  const currentState = read("current_state");
  if (!nextAction || !currentState) return;

  const task = typeof nextAction.task === "string" ? nextAction.task : currentState.task;

  if (Number.isInteger(nextAction.iteration) && Number.isInteger(currentState.iteration) &&
      nextAction.iteration !== currentState.iteration) {
    err("next_action ↔ current_state", currentState.__line ?? "?",
      `iteration: единое iteration нарушено (план ${nextAction.iteration}, состояние ${currentState.iteration})`);
  }
  if (Number.isInteger(currentState.iteration) && Number.isInteger(currentState.rework) &&
      currentState.rework !== currentState.iteration - 1) {
    err("current_state.yaml", currentState.__line ?? "?",
      `rework: rework = iteration − 1 (iteration ${currentState.iteration}, rework ${currentState.rework})`);
  }

  // единое iteration текущего раунда: план/состояние vs последняя запись
  // progress/receipts по той же задаче.
  const expected = Number.isInteger(currentState.iteration)
    ? currentState.iteration
    : Number.isInteger(nextAction.iteration)
      ? nextAction.iteration
      : null;
  if (expected == null) return;

  for (const kind of ["progress", "receipts"]) {
    const doc = read(kind);
    const recs = Array.isArray(doc) ? doc : [];
    const mine = recs.filter((r) => r && typeof r === "object" && (!task || r.task === task));
    if (!mine.length) continue;
    const last = mine[mine.length - 1];
    if (Number.isInteger(last.iteration) && last.iteration !== expected) {
      err(`${kind}.yaml ↔ current_state`, last.__line ?? "?",
        `iteration: единое iteration нарушено (состояние ${expected}, последняя запись ${kind} ${last.iteration})`);
    }
  }
}

// --- Прогон ------------------------------------------------------------

for (const kind of files) {
  const path = join(stateDir, `${kind}.yaml`);
  if (!existsSync(path)) {
    warn(`${kind}.yaml`, "—", "файл состояния отсутствует");
    continue;
  }
  let doc;
  try {
    doc = parseYaml(readFileSync(path, "utf-8"));
  } catch (e) {
    err(`${kind}.yaml`, "—", `ошибка разбора YAML: ${e.message}`);
    continue;
  }

  const recs = recordsOf(doc, kind);
  if (recs === null) {
    err(`${kind}.yaml`, doc.__line ?? "—",
      "ожидался список записей или контейнер (entries/actions/items/log/records/history)");
    continue;
  }
  if (Array.isArray(recs)) {
    recs.forEach((rec, idx) => checkRecord(`${kind}.yaml`, kind, rec, idx));
    if (kind === "progress") checkProgressInvariants(recs);
    if (kind === "receipts") checkReceiptsInvariants(recs);
  } else {
    checkRecord(`${kind}.yaml`, kind, recs, 0);
    if (kind === "next_action") checkNextAction(recs);
  }
}

if (!fileArg) crossCheck();

// --- Отчёт -------------------------------------------------------------

const ok = errors.length === 0;

if (asJson) {
  console.log(JSON.stringify({ ok, dir: stateDir, errors, warnings }, null, 2));
} else {
  console.log(`validate-state: ${stateDir}`);
  console.log(`  артефакты: ${files.join(", ")}`);
  const printGrouped = (list, tag) => {
    const groups = new Map();
    for (const item of list) {
      const key = `${item.file} — ${item.message}`;
      if (!groups.has(key)) groups.set(key, []);
      groups.get(key).push(item.line);
    }
    for (const [key, lines] of groups) {
      const shown = lines.filter((l) => l !== "?").slice(0, 3);
      const where = shown.length ? ` (строки ${shown.join(", ")}${lines.length > shown.length ? ", …" : ""})` : "";
      console.log(`  [${tag}] ${key}${where}`);
    }
  };
  printGrouped(errors, "ERROR");
  printGrouped(warnings, "WARN ");
  console.log(
    `  итог: ошибок ${errors.length}, предупреждений ${warnings.length} — ${ok ? "ОК" : "ЕСТЬ ОШИБКИ"}`,
  );
}

process.exit(ok ? 0 : 1);
