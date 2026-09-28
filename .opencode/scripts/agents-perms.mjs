#!/usr/bin/env node
// agents-perms.mjs — компактная сводка прав агентов из `opencode debug agents`.
//
// Зачем: сырой `opencode debug agents` печатает ~122 КБ JSON (18 агентов,
// включая полные `system`-промпты и `request`) — для ролей это дорого, а вывод
// режется token-guard. Скрипт оставляет только `permissions`/`steps`/`mode`:
// сводка ≈3–4 КБ (11 ролей).
//
// Канон: Q56/D51 (`review.md` §«Порог существенности» и «Доступные команды»);
// роли: validator, auditor.
//
// Использование:
//   node .opencode/scripts/agents-perms.mjs                  — сводка (роли из
//     `.opencode/agents`, права shell и edit)
//   node .opencode/scripts/agents-perms.mjs --role validator — все права роли
//   node .opencode/scripts/agents-perms.mjs --grep cargo     — поиск подстроки
//     по правам (ресурсам) всех ролей
//   node .opencode/scripts/agents-perms.mjs --json           — сводка в JSON
//   node .opencode/scripts/agents-perms.mjs --all            — вместе со
//     встроенными агентами (Build, Plan, Explore, …)
//
// Коды выхода: 0 — ok; 1 — вывод не JSON; 2 — ошибка запуска CLI/файла.

import { execSync } from 'node:child_process';
import { readdirSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const agentsDir = join(here, '..', 'agents');

function usage() {
  console.log(`Компактная сводка прав агентов (Q56/D51).

Использование:
  node .opencode/scripts/agents-perms.mjs [--role <имя>] [--grep <подстрока>] [--json] [--all]

  --role <имя>      все права одной роли (edit/read/shell/…)
  --grep <подстрока> строки прав, содержащие подстроку (без учёта регистра)
  --json            машиночитаемая сводка
  --all             включить встроенные агенты (Build, Compaction, Explore, …)
  --help, -h        эта справка`);
}

const args = process.argv.slice(2);
if (args.includes('--help') || args.includes('-h')) {
  usage();
  process.exit(0);
}

function argValue(flag) {
  const i = args.indexOf(flag);
  return i >= 0 && i + 1 < args.length ? args[i + 1] : undefined;
}

const roleName = argValue('--role');
const grepText = argValue('--grep');
const asJson = args.includes('--json');
const includeAll = args.includes('--all');

let raw;
try {
  raw = execSync('opencode debug agents', {
    encoding: 'utf8',
    maxBuffer: 64 * 1024 * 1024,
  });
} catch (e) {
  console.error('agents-perms: не удалось выполнить `opencode debug agents`:', e.message);
  process.exit(2);
}

let agents;
try {
  agents = JSON.parse(raw);
  if (!Array.isArray(agents)) throw new Error('ожидался массив');
} catch (e) {
  console.error('agents-perms: вывод CLI не распознан как JSON:', e.message, '| начало:', JSON.stringify(raw.slice(0, 120)));
  process.exit(1);
}

let repoAgents = [];
try {
  repoAgents = readdirSync(agentsDir)
    .filter((f) => f.endsWith('.md'))
    .map((f) => f.slice(0, -3));
} catch {
  repoAgents = [];
}

const inRepo = (name) => repoAgents.length === 0 || repoAgents.includes(name);
const selected = agents.filter((a) => includeAll || inRepo(a.name));

function permsOf(agent, action) {
  return (agent.permissions || []).filter((p) => p.action === action);
}

// Эффективный список (без boilerplate `resource: "*", effect: "deny"`).
function effective(agent, action) {
  return permsOf(agent, action).filter((p) => !(p.effect === 'deny' && p.resource === '*'));
}

function shellSummary(agent) {
  const eff = effective(agent, 'shell');
  const parts = [];
  for (const p of eff) {
    parts.push(`${p.effect === 'ask' ? 'ask' : 'allow'}:${p.resource}`);
  }
  return parts.join(' ');
}

function editSummary(agent) {
  return effective(agent, 'edit')
    .map((p) => p.resource)
    .join(' ');
}

if (roleName) {
  const a = agents.find((x) => x.name === roleName);
  if (!a) {
    console.error(`agents-perms: роль «${roleName}» не найдена. Доступные: ${agents.map((x) => x.name).join(', ')}`);
    process.exit(2);
  }
  console.log(`${a.name} [${a.mode}] steps=${a.steps ?? '-'} model=${a.model?.providerID ?? '?'}/${a.model?.id ?? '?'}`);
  for (const action of ['edit', 'read', 'shell', 'execute', 'webfetch', 'websearch', 'subagent', 'question', 'skill', 'external_directory']) {
    const list = permsOf(a, action);
    if (list.length === 0) continue;
    console.log(`  ${action}:`);
    for (const p of list) console.log(`    ${p.effect} ${p.resource}`);
  }
  process.exit(0);
}

if (grepText) {
  const needle = grepText.toLowerCase();
  let hits = 0;
  for (const a of selected) {
    for (const p of a.permissions || []) {
      if (p.resource.toLowerCase().includes(needle)) {
        console.log(`${a.name}: ${p.effect} ${p.action} ${p.resource}`);
        hits += 1;
      }
    }
  }
  console.log(`---`);
  console.log(`hits: ${hits}`);
  process.exit(0);
}

if (asJson) {
  const out = selected.map((a) => ({
    name: a.name,
    mode: a.mode,
    steps: a.steps ?? null,
    model: a.model ? `${a.model.providerID}/${a.model.id}` : null,
    edit: effective(a, 'edit').map((p) => `${p.effect}:${p.resource}`),
    shell: effective(a, 'shell').map((p) => `${p.effect}:${p.resource}`),
  }));
  console.log(JSON.stringify(out, null, 2));
  process.exit(0);
}

console.log(`agents: ${selected.length} из ${agents.length} (--all — все)`);
for (const a of selected) {
  const shell = shellSummary(a);
  const edit = editSummary(a);
  const line = `${a.name} [${a.mode}] steps=${a.steps ?? '-'} edit=${edit || '-'} shell=${shell || '-'}`;
  console.log(line);
}
