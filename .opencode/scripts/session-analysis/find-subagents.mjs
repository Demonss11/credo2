#!/usr/bin/env node
// find-subagents.mjs — переход от корневой сессии к субагентским: список
// дочерних сессий (вызовы инструмента `subagent`) по транскрипту родителя.
//
// Зачем: `opencode session list` показывает только top-level сессии; ID
// субагентских сессий живут в родительском транскрипте — в результате вызова
// `subagent` первой строкой идёт `<subagent sessionID="ses_…" state="…">`.
//
// Использование:
//   node find-subagents.mjs <ses_parent | parent.json> [--json] [--fresh] [--export]
//
// Аргументы:
//   <ses_parent>   ID родительской сессии (ses_…): берётся готовый экспорт из
//                  <temp>/opencode/sessions/<ses>.json; если его нет (или задан
//                  --fresh) — экспорт выполняется export-session.mjs
//   parent.json    путь к готовому экспорту (JSON)
//   --json         машинный вывод: JSON-массив дочерних сессий
//   --fresh        переэкспортировать родителя, даже если файл уже есть
//   --export       дополнительно выгрузить транскрипты всех дочерних сессий
//                  (export-session.mjs в <temp>/opencode/sessions/)
//
// Пример (pwsh, из корня репозитория):
//   node .opencode/scripts/session-analysis/find-subagents.mjs ses_f1b8398d2ffe4VC7LhwgnYggwl
//   node .opencode/scripts/session-analysis/find-subagents.mjs ses_f1b8398d2ffe4VC7LhwgnYggwl --json
//
// Коды выхода: 0 — ok; 1 — JSON/разбор; 2 — использование/запуск.
// Требуется работающий сервис OpenCode — только при выгрузке (--fresh/--export).

import { existsSync, readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { tmpdir } from 'node:os';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { tryRepair } from './fix-encoding.mjs';

const HERE = dirname(fileURLToPath(import.meta.url));
const EXPORT_SCRIPT = join(HERE, 'export-session.mjs');
const SESSIONS_DIR = join(tmpdir(), 'opencode', 'sessions');

function usage() {
  console.log(`Список субагентских (дочерних) сессий по транскрипту родительской.

Использование:
  node find-subagents.mjs <ses_parent | parent.json> [--json] [--fresh] [--export]

  <ses_parent>   ID родительской сессии (ses_…): готовый экспорт из
                 <temp>/opencode/sessions/<ses>.json, иначе — экспорт через
                 export-session.mjs
  parent.json    путь к готовому экспорту
  --json         JSON-вывод (массив дочерних сессий)
  --fresh        переэкспортировать родителя принудительно
  --export       выгрузить транскрипты всех дочерних сессий

Пример:
  node .opencode/scripts/session-analysis/find-subagents.mjs ses_f1b8398d2ffe4VC7LhwgnYggwl`);
}

const argv = process.argv.slice(2);
const flags = new Set(argv.filter((a) => a.startsWith('--')));
const positional = argv.filter((a) => !a.startsWith('--'));
if (flags.has('--help') || flags.has('-h')) {
  usage();
  process.exit(0);
}
for (const f of flags) {
  if (!['--json', '--fresh', '--export'].includes(f)) {
    console.error(`Неизвестный флаг: ${f}`);
    usage();
    process.exit(2);
  }
}
if (positional.length === 0) {
  usage();
  process.exit(2);
}

const src = positional[0];
const asJson = flags.has('--json');
const fresh = flags.has('--fresh');
const exportChildren = flags.has('--export');

let jsonPath = src;
if (/^ses_[A-Za-z0-9]+$/.test(src)) {
  jsonPath = join(SESSIONS_DIR, `${src}.json`);
  if (fresh || !existsSync(jsonPath)) {
    const r = spawnSync(process.execPath, [EXPORT_SCRIPT, src], { stdio: 'inherit' });
    if (r.status !== 0) process.exit(r.status || 2);
  } else {
    console.error(`Использую готовый экспорт: ${jsonPath} (для свежего — --fresh)`);
  }
} else if (!existsSync(jsonPath)) {
  console.error(`Файл не найден: ${jsonPath}`);
  process.exit(2);
}

let raw;
try {
  raw = readFileSync(jsonPath, 'utf8');
} catch (e) {
  console.error(`Не читается ${jsonPath}: ${e.message}`);
  process.exit(2);
}
if (raw.charCodeAt(0) === 0xfeff) raw = raw.slice(1);
const rep = tryRepair(raw);
if (rep.status === 'repaired') raw = rep.candidate;
else if (rep.status === 'ambiguous') {
  console.error(`Внимание: в ${jsonPath} возможны следы CP866 (см. fix-encoding.mjs); продолжаю как есть.`);
}

let data;
try {
  data = JSON.parse(raw);
} catch (e) {
  console.error(`JSON не разбирается: ${e.message}`);
  process.exit(1);
}

// Дочерние сессии: вызовы инструмента subagent (task — совместимость).
const calls = [];
(data.messages || []).forEach((m, i) => {
  for (const pt of m.content || []) {
    if (pt.type !== 'tool' || !['subagent', 'task'].includes(pt.name)) continue;
    const st = pt.state || {};
    const text = Array.isArray(st.content)
      ? st.content.map((c) => (c && c.text) || '').join('\n')
      : typeof st.content === 'string'
        ? st.content
        : '';
    const tag = text.match(/<subagent\s+sessionID="(ses_[A-Za-z0-9]+)"(?:\s+state="([^"]+)")?/);
    const id = tag ? tag[1] : (text.match(/ses_[A-Za-z0-9]+/) || [])[0] || null;
    calls.push({
      index: i,
      id,
      tool: pt.name,
      status: st.status || '?',
      childState: tag ? tag[2] || null : null,
      agent: (st.input && st.input.agent) || '',
      description: (st.input && st.input.description) || '',
    });
  }
});

const children = [];
const seen = new Map();
for (const c of calls) {
  if (c.id && seen.has(c.id)) {
    seen.get(c.id).repeats++;
    continue;
  }
  const rec = { ...c, repeats: 1 };
  children.push(rec);
  if (c.id) seen.set(c.id, rec);
}

const info = data.info || {};
if (info.parentID) {
  console.error(
    `Внимание: ${info.id} — дочерняя сессия (родитель ${info.parentID}); ` +
      `для списка сиблингов запустите скрипт на родителе.`
  );
}

if (asJson) {
  console.log(
    JSON.stringify(
      {
        session: info.id || null,
        agent: info.agent || null,
        parentID: info.parentID || null,
        calls: calls.length,
        children,
      },
      null,
      2
    )
  );
} else {
  console.log(`Сессия: ${info.id || src} · агент: ${info.agent || '?'} · родитель: ${info.parentID || '—'}`);
  console.log(`Вызовов subagent: ${calls.length} · уникальных дочерних сессий: ${children.length}`);
  if (children.length === 0) {
    console.log(
      'Дочерних сессий не найдено. Проверьте, что это корневая сессия прогона; ' +
        'ID также могут быть в ленте задачи/progress.yaml.'
    );
  } else {
    const w = (key, head) => Math.max(head.length, ...children.map((c) => String(c[key] ?? '').length));
    const wId = w('id', 'сессия');
    const wAgent = w('agent', 'агент');
    const wDesc = Math.min(60, w('description', 'описание'));
    console.log(
      ['#', 'сессия'.padEnd(wId), 'агент'.padEnd(wAgent), 'статус'.padEnd(9), 'msg'.padEnd(6), 'описание'.slice(0, wDesc)].join('  ')
    );
    children.forEach((c, n) => {
      console.log(
        [
          String(n + 1).padEnd(2),
          (c.id || '(id не найден)').padEnd(wId),
          c.agent.padEnd(wAgent),
          String(c.childState || c.status).padEnd(9),
          ('[' + c.index + ']').padEnd(6),
          c.description.slice(0, wDesc) + (c.repeats > 1 ? `  (×${c.repeats})` : ''),
        ].join('  ')
      );
    });
    console.log('');
    console.log('Дальше: export-session.mjs <сессия> → analyze-session.mjs <json>');
  }
}

if (exportChildren && children.length > 0) {
  console.log('');
  console.log('== Выгрузка транскриптов дочерних сессий ==');
  for (const c of children) {
    if (!c.id) continue;
    const r = spawnSync(process.execPath, [EXPORT_SCRIPT, c.id], { stdio: 'inherit' });
    if (r.status !== 0) console.error(`Не удалось выгрузить ${c.id} (код ${r.status}).`);
  }
}
