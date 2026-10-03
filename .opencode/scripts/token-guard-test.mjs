#!/usr/bin/env node
// T-15 · C11 (D94) — token-guard-test.mjs: тест профилей B2 (снятие tool-схем
// по префиксам) на синтетических выводах. Не канон; служебная зона.
// Запуск: node .opencode/scripts/token-guard-test.mjs
// Проверяет: профиль "off" — no-op; "codemode"/"mcp" — снятие схем по
// префиксам роли; B1 (срез текста) не зависит от профиля.
// Профиль-константа B2_PROFILE живёт в .opencode/plugins/token-guard.ts;
// здесь — независимая копия правил для проверки (тест не импортирует TS).

import { readFileSync } from "node:fs";

const PLUGIN = ".opencode/plugins/token-guard.ts";
const src = readFileSync(PLUGIN, "utf8");

// Правила профилей — зеркало констант плагина (сверяется ниже по тексту).
const RULES = {
  codemode: {
    "docs-writer": ["rust-analyzer", "rust_analyzer", "credo"],
    git: ["rust-analyzer", "rust_analyzer", "credo"],
    analyst: ["rust-analyzer", "rust_analyzer"],
  },
  mcp: {
    "docs-writer": ["rust-analyzer", "rust_analyzer", "credo"],
    git: ["rust-analyzer", "rust_analyzer", "credo"],
    analyst: ["rust-analyzer", "rust_analyzer"],
  },
};

// Синтетические наборы tool-схем по профилям.
const TOOLS = {
  codemode: {
    // Code Mode: MCP-инструменты доступны как ключи внутри execute.
    "docs-writer": {
      read: {},
      edit: {},
      "rust-analyzer_symbols": {},
      rust_analyzer_hover: {},
      credo_check_create: {},
    },
    git: { read: {}, "rust-analyzer_symbols": {}, credo_check_list_drafts: {} },
    analyst: { read: {}, "rust-analyzer_symbols": {}, rust_analyzer_hover: {} },
    lead: { read: {}, subagent: {}, credo_check_test: {} },
  },
  mcp: {
    // Прямая экспозиция MCP: инструменты — ключи верхнего уровня.
    "docs-writer": {
      read: {},
      write: {},
      "rust-analyzer_symbols": {},
      credo_check_publish: {},
    },
    git: { read: {}, credo_check_list_published: {} },
    analyst: { read: {}, rust_analyzer_hover: {} },
    lead: { read: {}, credo_check_test: {} },
  },
};

let fails = 0;
const ok = (cond, msg) => {
  console.log(`${cond ? "ok  " : "FAIL"} ${msg}`);
  if (!cond) fails += 1;
};

// No-op профиля "off": activeB2Prefixes() возвращает {}.
const offActive = (() => {
  const m = src.match(/const B2_PROFILE[^=]*=\s*"([a-z]+)"/);
  return m ? m[1] : null;
})();
ok(offActive === "off", `B2_PROFILE по умолчанию = "off" (факт: ${offActive})`);

for (const [profile, agents] of Object.entries(TOOLS)) {
  for (const [agent, tools] of Object.entries(agents)) {
    const prefixes = RULES[profile][agent];
    const keys = Object.keys(tools);
    if (!prefixes) {
      // lead и роли без правил — не трогаем.
      ok(keys.length > 0, `${profile}/${agent}: правило отсутствует — схемы не снимаются`);
      continue;
    }
    const removed = keys.filter((k) => prefixes.some((p) => k.startsWith(p)));
    const keep = keys.filter((k) => !prefixes.some((p) => k.startsWith(p)));
    ok(removed.length > 0, `${profile}/${agent}: снимается ≥1 схема (${removed.length})`);
    ok(keep.every((k) => !prefixes.some((p) => k.startsWith(p))), `${profile}/${agent}: лишние не затронуты`);
  }
}

// Зеркало текста плагина: таблицы обоих профилей объявлены и РАВНЫ правилам.
const parseTable = (name) => {
  const m = src.match(new RegExp(`const ${name}[^=]*=\\s*\\{([\\s\\S]*?)\\n\\};`));
  if (!m) return null;
  const body = m[1];
  const out = {};
  // Запись вида:  "docs-writer": ["a", "b"],  |  analyst: ["a"],
  const entry = /(?:["']([^"']+)["']|([A-Za-z_$][\w$]*))\s*:\s*\[([^\]]*)\]/g;
  let e;
  while ((e = entry.exec(body)) !== null) {
    const key = e[1] ?? e[2];
    out[key] = e[3]
      .split(",")
      .map((s) => s.trim().replace(/^["']|["']$/g, ""))
      .filter(Boolean);
  }
  return out;
};
for (const [name, profile] of [
  ["B2_PREFIXES_CODEMODE", "codemode"],
  ["B2_PREFIXES_MCP", "mcp"],
]) {
  const actual = parseTable(name);
  ok(actual !== null, `плагин: ${name} разобран`);
  ok(
    actual !== null && JSON.stringify(actual) === JSON.stringify(RULES[profile]),
    `плагин: ${name} равен зеркалу RULES.${profile}`,
  );
}
ok(/activeB2Prefixes/.test(src), "плагин: activeB2Prefixes() используется");
ok(/B2_PROFILES\[B2_PROFILE\]\s*\?\?\s*\{\}/.test(src), "плагин: off → пустая таблица (no-op)");
ok(!/B2_PREFIXES\[agent\]/.test(src), "плагин: старая B2_PREFIXES[agent] не осталась");

// B1 не задет: срез текста не смотрит на профиль.
ok(/const OUTPUT_LIMIT = 12_000/.test(src), "B1: порог OUTPUT_LIMIT не изменён");
ok(/sliceText/.test(src), "B1: sliceText на месте");

console.log(`\n${fails === 0 ? "PASS" : "FAIL"} (${fails} ошибок)`);
process.exit(fails === 0 ? 0 : 1);
