#!/usr/bin/env node
// git-check.mjs — read-only сводка состояния репозитория для роли git.
// Заменяет серию одиночных inspection-команд (status/log/--cached) одним вызовом.
//
// Использование (из корня репозитория или откуда угодно):
//   node .opencode/scripts/git-check.mjs              — ветка/ahead, HEAD, счётчики дерева
//   node .opencode/scripts/git-check.mjs --staged     — + staged: список путей и счёт
//   node .opencode/scripts/git-check.mjs --expect=N   — сверить число путей (дерева; со --staged — staged)
//   node .opencode/scripts/git-check.mjs --staged --expect=86
//
// Выход: 0 — ок; 1 — расхождение с --expect или ошибка git.
// Скрипт НЕ выполняет изменяющих команд (add/commit/push) — это по-прежнему `git`-роль под `ask`.

import { execFileSync } from "node:child_process";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const ROOT = resolve(__dirname, "../..");

function git(args) {
  return execFileSync("git", args, {
    cwd: ROOT,
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
  }).trim();
}

function gitSafe(args) {
  try {
    return { ok: true, out: git(args) };
  } catch (err) {
    const msg = String(err.stderr || err.message || err).trim();
    return { ok: false, out: msg };
  }
}

const argv = process.argv.slice(2);
const staged = argv.includes("--staged");
const expectArg = argv.find((a) => a.startsWith("--expect="));
const expect = expectArg ? Number.parseInt(expectArg.split("=")[1], 10) : null;

let failed = false;
const out = (s) => console.log(s);
const fail = (s) => {
  failed = true;
  console.error(s);
};

// 1. Ветка и позиция относительно origin
const sb = gitSafe(["status", "-sb"]);
if (!sb.ok) {
  console.error(`[git-check] git status: ${sb.out}`);
  process.exit(1);
}
out(`branch: ${sb.out.split("\n")[0]}`);

// 2. HEAD (идемпотентность: одна проверка)
const head = gitSafe(["log", "-1", "--oneline"]);
if (!head.ok) fail(`[git-check] git log -1: ${head.out}`);
out(`head:   ${head.out}`);

// 3. Рабочее дерево (porcelain)
const porcelain = gitSafe(["status", "--porcelain"]);
if (!porcelain.ok) {
  console.error(`[git-check] git status --porcelain: ${porcelain.out}`);
  process.exit(1);
}
const rows = porcelain.out ? porcelain.out.split("\n").filter(Boolean) : [];
const stat = { M: 0, D: 0, A: 0, U: 0, other: 0 };
for (const r of rows) {
  const code = r.slice(0, 2);
  if (code.includes("?")) stat.U += 1;
  else if (code.includes("M")) stat.M += 1;
  else if (code.includes("D")) stat.D += 1;
  else if (code.includes("A")) stat.A += 1;
  else stat.other += 1;
}
out(
  `tree:   ${rows.length} путей — M ${stat.M}, D ${stat.D}, A ${stat.A}, ?? ${stat.U}` +
    (stat.other ? `, прочее ${stat.other}` : "")
);

// 4. Staged (опция)
let stagedCount = 0;
if (staged) {
  const cached = gitSafe(["diff", "--cached", "--name-status"]);
  if (!cached.ok) {
    console.error(`[git-check] git diff --cached: ${cached.out}`);
    process.exit(1);
  }
  const cRows = cached.out ? cached.out.split("\n").filter(Boolean) : [];
  stagedCount = cRows.length;
  out(`staged: ${stagedCount} путей`);
  for (const r of cRows) out(`  ${r}`);
}

// 5. Сверка с ожиданием
const actual = staged ? stagedCount : rows.length;
if (expect !== null && Number.isInteger(expect)) {
  if (actual === expect) out(`expect: ${expect} — совпало`);
  else fail(`[git-check] ОЖИДАЛОСЬ ${expect}, ФАКТ ${actual}`);
}

process.exit(failed ? 1 : 0);
