#!/usr/bin/env node
// agent-report.mjs — полный цикл разбора одного агента по корневой сессии.
//
// Вход:  <ses_корень | parent.json> <агент>
// Выход (по умолчанию в <temp>/opencode/sessions/):
//   ses_<ребёнок>.json        — транскрипт (export-session.mjs);
//   ses_<ребёнок>.report.txt  — факты (analyze-session.mjs);
//   ses_<ребёнок>.draft.md    — черновик служебного отчёта: факты заполнены,
//                               оценка/выводы/предложения — по шаблону
//                               session-report-template.md (рядом с тулкитом).
//
// Сессия ищется по транскрипту корня (вызовы инструмента `subagent`);
// если у агента несколько сессий (resume, повторные пакеты) — обрабатываются
// все, `--last` оставляет только последнюю.
//
// Использование:
//   node agent-report.mjs <ses_корень | parent.json> <агент>
//        [--last] [--list] [--fresh] [--facts-only] [--out <dir>] [--json]
//
// Коды выхода: 0 — ok; 1 — агент/сессия не найдены или ошибка разбора; 2 — usage.

import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { tmpdir } from "node:os";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { tryRepair } from "./fix-encoding.mjs";

const HERE = dirname(fileURLToPath(import.meta.url));
const REPO_ROOT = join(HERE, "..", "..", ".."); // .opencode/scripts/session-analysis → корень
const DEFAULT_OUT = join(tmpdir(), "opencode", "sessions");
const TOOLS = {
  find: join(HERE, "find-subagents.mjs"),
  export: join(HERE, "export-session.mjs"),
  analyze: join(HERE, "analyze-session.mjs"),
};

function usage() {
  console.log(`Полный цикл разбора агента: корневая сессия + агент → отчёты.

Использование:
  node agent-report.mjs <ses_корень | parent.json> <агент> [опции]

Аргументы:
  <ses_корень>  ID корневой сессии (ses_…) или путь к её экспорту (JSON)
  <агент>       имя роли: coder, tester, validator, auditor, git, …

Опции:
  --last         только последняя сессия агента (по умолчанию — все)
  --list         показать найденные сессии агента и выйти (отчёты не строятся)
  --fresh        переэкспортировать корень и детей (иначе — готовые JSON)
  --facts-only   без черновика отчёта (только транскрипт и факты)
  --out <dir>    каталог вывода (по умолчанию <temp>/opencode/sessions)
  --json         итог машинно (JSON-массив обработанных сессий)
  --help, -h     эта справка

Примеры:
  node .opencode/scripts/session-analysis/agent-report.mjs ses_f1b8398d2ffe4VC7LhwgnYggwl git --list
  node .opencode/scripts/session-analysis/agent-report.mjs ses_f1b8398d2ffe4VC7LhwgnYggwl tester
  node .opencode/scripts/session-analysis/agent-report.mjs ses_f1b8398d2ffe4VC7LhwgnYggwl git --last --fresh

Дальше: дополнить черновик по session-report-template.md (рядом с тулкитом) и
сохранить как docs/analysis/<T-XX>-run<N>-<роль>-session.md.`);
}

function parseArgs(argv) {
  const opts = { last: false, list: false, fresh: false, factsOnly: false, json: false, out: null };
  const pos = [];
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    switch (a) {
      case "--last": opts.last = true; break;
      case "--list": opts.list = true; break;
      case "--fresh": opts.fresh = true; break;
      case "--facts-only": opts.factsOnly = true; break;
      case "--json": opts.json = true; break;
      case "--out":
        opts.out = argv[++i];
        if (!opts.out) {
          console.error("Ошибка: после --out нужен путь к каталогу.");
          process.exit(2);
        }
        break;
      case "--help":
      case "-h": opts.help = true; break;
      default:
        if (a.startsWith("--")) {
          console.error(`Неизвестный флаг: ${a}`);
          usage();
          process.exit(2);
        }
        pos.push(a);
    }
  }
  return { opts, pos };
}

function loadSession(path) {
  let raw = readFileSync(path, "utf8");
  if (raw.charCodeAt(0) === 0xfeff) raw = raw.slice(1);
  const rep = tryRepair(raw);
  if (rep.status === "repaired") raw = rep.candidate;
  return JSON.parse(raw);
}

const RU = new Intl.DateTimeFormat("ru-RU", {
  timeZone: "Europe/Moscow",
  year: "numeric", month: "2-digit", day: "2-digit",
  hour: "2-digit", minute: "2-digit", second: "2-digit",
  hour12: false,
});
function msk(t) {
  if (!t) return "?";
  try {
    return RU.format(new Date(t)) + " (+03:00)";
  } catch {
    return new Date(t).toISOString();
  }
}
const min = (ms) => (ms / 60000).toFixed(1).replace(".", ",");

/** Факты из экспорта сессии — для сводки черновика. */
function collectFacts(data, agentName) {
  const I = data.info || {};
  const msgs = data.messages || [];
  const agent = I.agent || agentName;

  const prompts = [];
  msgs.forEach((m, i) => {
    if (m.type === "user" && typeof m.text === "string" && m.text.trim()) {
      prompts.push({ i, at: m.time && m.time.created, len: m.text.length });
    }
  });

  const segs = [];
  let cur = null;
  for (const [i, m] of msgs.entries()) {
    if (m.type === "user" && (m.text || "").trim()) {
      cur = { prompt: i, steps: 0, tools: 0, reason: 0, first: null, last: null };
      segs.push(cur);
      continue;
    }
    if (!Array.isArray(m.content)) continue;
    if (!cur) {
      cur = { prompt: null, steps: 0, tools: 0, reason: 0, first: null, last: null };
      segs.push(cur);
    }
    cur.steps++;
    const t = m.time || {};
    if (t.created) cur.first = cur.first === null ? t.created : cur.first;
    if (t.completed) cur.last = t.completed;
    for (const pt of m.content) {
      if (pt.type === "tool") cur.tools++;
      else if (pt.type === "reasoning") cur.reason += (pt.text || "").length;
    }
  }

  const toolCounts = {};
  const cuts = [];
  const rej = [];
  const fin = {};
  const stopTurns = [];
  let assistants = 0;
  let modelMs = 0;
  let emptyStops = 0;
  msgs.forEach((m, i) => {
    if (!Array.isArray(m.content)) return;
    assistants++;
    fin[m.finish || "?"] = (fin[m.finish || "?"] || 0) + 1;
    const t = m.time || {};
    if (t.created && t.completed) modelMs += t.completed - t.created;
    const text = m.content.filter((p) => p.type === "text").map((p) => p.text || "").join("");
    if (m.finish === "stop") {
      stopTurns.push({ i, len: text.length });
      if (!text.trim()) emptyStops++;
    }
    for (const pt of m.content) {
      if (pt.type !== "tool") continue;
      const st = pt.state || {};
      toolCounts[pt.name] = (toolCounts[pt.name] || 0) + 1;
      let out = "";
      if (Array.isArray(st.content)) out = st.content.map((c) => (c && c.text) || "").join("\n");
      else if (typeof st.content === "string") out = st.content;
      const mk = out.match(/\[token-guard\] срез: опущено ~(\d+) байт из (\d+)/);
      if (mk) cuts.push({ i, tool: pt.name, omitted: +mk[1], total: +mk[2] });
      if (st.error && st.error.type === "permission.rejected") {
        const cmd = st.input && (st.input.command || st.input.path || st.input.filePath);
        rej.push({ i, tool: pt.name, cmd: cmd || "" });
      }
    }
  });

  let limit = null;
  try {
    const text = readFileSync(join(REPO_ROOT, ".opencode", "agents", `${agent}.md`), "utf8");
    const m = text.match(/^steps:\s*(\d+)/m);
    if (m) limit = +m[1];
  } catch {
    /* роль без файла — лимит неизвестен */
  }

  const created = I.time && I.time.created;
  const idle = I.time && I.time.idle;
  const tok = I.tokens || {};
  const cache = tok.cache || {};
  const taskId = ((I.title || "").match(/T-\d+/) || [])[0] || null;

  return {
    info: I, agent, prompts, segs, toolCounts, cuts, rej, fin, stopTurns,
    assistants, modelMs, emptyStops, limit,
    wallMs: created && idle ? idle - created : null,
    tokens: tok, cache, taskId,
  };
}

/** Черновик служебного отчёта: факты заполнены, анализ — по шаблону. */
function buildDraft(data, agentName, paths) {
  const f = collectFacts(data, agentName);
  const I = f.info;
  const L = [];
  const P = (s) => L.push(s);

  P(`# Разбор сессии \`${f.agent}\` — ${I.title || "(без названия)"} (черновик)`);
  P("");
  P("> Черновик `agent-report.mjs`: **факты автозаполнены**; оценку, выводы и");
  P("> предложения дописать по шаблону `session-report-template.md` (рядом с тулкитом).");
  P("> Итоговое имя: `docs/analysis/<T-XX>-run<N>-<роль>-session.md`.");
  P("> Улика, не канон (Q41); артефакты разбора — вне репозитория.");
  P("");
  P(`- **Сессия:** \`${I.id || "?"}\` (агент \`${f.agent}\`, модель \`${(I.model && I.model.id) || "?"}\`; parent — \`${I.parentID || "—"}\`)`);
  P(`- **Окно:** ${msk(I.time && I.time.created)} → ${msk(I.time && I.time.idle)}; заголовок: «${I.title || "—"}»`);
  P("");
  P("## 1. Сводка (авто)");
  P("");
  P("| Метрика | Значение |");
  P("|---|---|");
  P(`| Длительность | ${f.wallMs ? min(f.wallMs) + " мин (created→idle)" : "?"}; модельного времени ${min(f.modelMs)} мин |`);
  P(`| Промпты | ${f.prompts.length}: ${f.prompts.map((p) => `[${p.i}] ${msk(p.at).replace(" (+03:00)", "")}`).join("; ") || "—"} |`);
  P(`| Ходы | ${f.assistants} ассистентских; сегменты: ${f.segs.map((s, k) => `seg${k} steps=${s.steps} tools=${s.tools}`).join("; ")}; лимит \`steps\`: ${f.limit === null ? "?" : f.limit} |`);
  const toolStr = Object.entries(f.toolCounts).sort((a, b) => b[1] - a[1]).map(([n, c]) => `${n} ${c}`).join(", ");
  P(`| Инструменты (${Object.values(f.toolCounts).reduce((a, b) => a + b, 0)} вызовов) | ${toolStr || "—"}; \`permission.rejected\` — ${f.rej.length} |`);
  P(`| Срезы \`token-guard\` | ${f.cuts.length}${f.cuts.length ? ": " + f.cuts.map((c) => `[${c.i}] ${c.tool} ~${c.omitted} из ${c.total}`).join("; ") : ""} |`);
  P(`| Токены/цена | вх. ${f.tokens.input ?? "?"} · вых. ${f.tokens.output ?? "?"} · reasoning ${f.tokens.reasoning ?? "?"} · cache read ${f.cache.read ?? "?"}; $${typeof I.cost === "number" ? I.cost.toFixed(4) : "?"} |`);
  P(`| Финал | \`finish\`: ${JSON.stringify(f.fin)}; пустых финалов — ${f.emptyStops} |`);
  P("");
  P("## 2. Хронология и точки останова (заготовка)");
  P("");
    P(`- Сегменты: ${f.segs.map((s, k) => `seg${k}${s.prompt !== null ? ` [prompt@${s.prompt}]` : ""}: steps=${s.steps}, tools=${s.tools}, ${msk(s.first)} → ${msk(s.last)}, reasoning ${s.reason} симв`).join("; ") || "—"}.`);
  P(`- Полные тексты промптов — \`report.txt\` §USER MESSAGES; таблица сообщений — §MESSAGES; пограничные тексты — §BOUNDARY TEXTS.`);
  P(`- Финалы \`stop\`: ${f.stopTurns.map((s) => `[${s.i}] textLen=${s.len}`).join("; ") || "—"}; \`finish=error\`: ${Object.entries(f.fin).find(([k]) => k === "error") ? "есть" : "нет"}.`);
  P("- <дописать ход событий, обрывы/продолжения и точки останова>.");
  P("");
  P("## 3. Ошибки, отказы, срезы (факты)");
  P("");
  P(`- \`permission.rejected\` (${f.rej.length}): ${f.rej.map((r) => `[${r.i}] ${r.tool} \`${r.cmd}\``).join("; ") || "—"}.`);
  P(`- Срезы \`token-guard\` (${f.cuts.length}): ${f.cuts.map((c) => `[${c.i}] ${c.tool} — опущено ~${c.omitted} Б из ${c.total}`).join("; ") || "—"}.`);
  P("- <дописать причины отказов по канону, компенсацию срезов, технические сбои>.");
  P("");
  P("## 4. Оценка качества работы");
  P("");
  P("- <канон роли: зоны записи/границы; содержание по типу роли; протокол/чекпойнты; экономика>.");
  P("");
  P("## 5. Сверка с артефактами");
  P("");
  P(`- <лента${f.taskId ? ` \`.opencode/mail/${f.taskId}.md\`` : " \`.opencode/mail/<T-XX>.md\`"}, память \`.opencode/memory/${f.agent}.md\`, state \`.opencode/state/current/{progress,receipts}.yaml\`, досье, приёмка, коммиты (\`git show --stat\`)>`);
  P("");
  P("## 6. Выводы (факт → оценка → предложение)");
  P("");
  P("1. <3–7 пунктов, каждое — с последствием>");
  P("");
  P("## 7. Ответы на вопросы разбора");
  P("");
  P(`- **Хватает ли \`${f.agent}\` лимита \`${f.limit === null ? "?" : f.limit}\`?** <фактически ${f.assistants} ходов; запас; менять/не менять>.`);
  P("- **Воспроизводимость потери отчётности?** <была/нет; как восстановлено>.");
  P("");
  P("## 8. Предложения к внесению (адресация)");
  P("");
  P("- <меморандум (F-уточнения) — сервисная сессия; реестр (кандидат F-записи) — `migrator`; канон/правила — сервисная сессия>.");
  P("");
  P("## Приложение. Артефакты разбора");
  P("");
  P(`- Транскрипт: \`${paths.json}\``);
  P(`- Факты: \`${paths.report}\``);
  P(`- Черновик: \`${paths.draft}\``);
  P(`- \`[i]\` — индекс сообщения в экспорте (${data.messages ? data.messages.length : "?"} шт.), не шаг модели.`);
  P("");
  return L.join("\n");
}

// ---------- CLI ----------

const { opts, pos } = parseArgs(process.argv.slice(2));
if (opts.help) {
  usage();
  process.exit(0);
}
if (pos.length < 2) {
  usage();
  process.exit(2);
}
const [root, agentArg] = pos;
const agent = agentArg.trim();

const findArgs = [root, "--json"];
if (opts.fresh) findArgs.push("--fresh");
const found = spawnSync(process.execPath, [TOOLS.find, ...findArgs], {
  encoding: "utf8",
  maxBuffer: 64 * 1024 * 1024,
});
if (found.error) {
  console.error(`Не запустился find-subagents.mjs: ${found.error.message}`);
  process.exit(2);
}
if (found.status !== 0) {
  console.error((found.stderr || "").trim() || `find-subagents.mjs завершился с кодом ${found.status}`);
  process.exit(found.status || 1);
}
let listing;
try {
  listing = JSON.parse(found.stdout);
} catch (e) {
  // stdout мог содержать служебные строки авто-выгрузки корня — берём с "{"
  const at = (found.stdout || "").indexOf("{");
  try {
    listing = JSON.parse((found.stdout || "").slice(at));
  } catch (e2) {
    console.error(`find-subagents.mjs: вывод не JSON (${e.message}; ${e2.message}).`);
    process.exit(1);
  }
}

const norm = (s) => (s || "").trim().toLowerCase();
const matches = (listing.children || [])
  .filter((c) => c.id && norm(c.agent) === norm(agent))
  .sort((a, b) => a.index - b.index);

if (matches.length === 0) {
  const avail = [...new Set((listing.children || []).map((c) => c.agent).filter(Boolean))];
  console.error(`У агента «${agent}» нет дочерних сессий (корень: ${listing.session || root}).`);
  console.error(`Доступные агенты: ${avail.join(", ") || "—"}`);
  process.exit(1);
}

if (opts.list) {
  const shown = opts.last ? [matches[matches.length - 1]] : matches;
  console.log(
    `Корень: ${listing.session || root} · агент: ${agent} · сессий: ${shown.length}` +
      (opts.last ? " (последняя)" : "")
  );
  for (const m of shown) {
    console.log(`  ${m.id}  [${m.index}]  ${m.childState || m.status}  ×${m.repeats}  ${m.description || ""}`);
  }
  process.exit(0);
}

const selected = opts.last ? [matches[matches.length - 1]] : matches;
const outDir = opts.out || DEFAULT_OUT;
mkdirSync(outDir, { recursive: true });

const results = [];
for (const m of selected) {
  const jsonPath = join(outDir, `${m.id}.json`);
  if (opts.fresh || !existsSync(jsonPath)) {
    const e = spawnSync(process.execPath, [TOOLS.export, m.id, jsonPath], { stdio: "inherit" });
    if (e.status !== 0) process.exit(e.status || 2);
  } else {
    console.error(`Готовый экспорт: ${jsonPath} (для свежего — --fresh)`);
  }

  const reportPath = join(outDir, `${m.id}.report.txt`);
  const a = spawnSync(process.execPath, [TOOLS.analyze, jsonPath, reportPath], { stdio: "inherit" });
  if (a.status !== 0) process.exit(a.status || 2);

  let draftPath = null;
  if (!opts.factsOnly) {
    draftPath = join(outDir, `${m.id}.draft.md`);
    const data = loadSession(jsonPath);
    const draft = buildDraft(data, agent, { json: jsonPath, report: reportPath, draft: draftPath });
    writeFileSync(draftPath, draft, "utf8");
  }

  results.push({
    session: m.id,
    agent: m.agent || agent,
    calls: m.repeats,
    description: m.description || "",
    json: jsonPath,
    report: reportPath,
    draft: draftPath,
  });
}

if (opts.json) {
  console.log(JSON.stringify(results, null, 2));
} else {
  console.log("");
  console.log(`Готово: сессий обработано ${results.length} (корень: ${listing.session || root}).`);
  for (const r of results) {
    console.log(`- ${r.session}${r.description ? ` («${r.description}»)`: ""}`);
    console.log(`    факты:   ${r.report}`);
    if (r.draft) console.log(`    черновик: ${r.draft}`);
  }
  console.log("Дальше: дополнить черновик по session-report-template.md (рядом с тулкитом).");
}
