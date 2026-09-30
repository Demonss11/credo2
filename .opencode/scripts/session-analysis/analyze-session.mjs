#!/usr/bin/env node
// analyze-session.mjs — сбор фактов по транскрипту сессии OpenCode в текстовый
// отчёт для служебного разбора (docs/analysis/<T-XX>-runN-<роль>-session.md).
//
// Что извлекается: хронология и сегменты (по промптам), расход шагов,
// инструменты и их статусы, отказы прав (permission.rejected), срезы
// token-guard, аварийные финалы (finish=error/пустой stop), полные тексты
// промптов и хвостовых ходов, чтения/правки по путям, вызовы Code Mode.
//
// Использование:
//   node analyze-session.mjs <session.json> [out.txt]
//
// По умолчанию отчёт пишется рядом с JSON: <session>.report.txt.
// Скрипт сам распознаёт CP866-кракозябры и восстанавливает текст в памяти
// (файл не перезаписывается; для файла используйте fix-encoding.mjs).

import { readFileSync, writeFileSync } from 'node:fs';
import { basename, dirname, join } from 'node:path';
import { tryRepair } from './fix-encoding.mjs';

const args = process.argv.slice(2);
if (args.length === 0 || args.includes('--help') || args.includes('-h')) {
  console.log(`Сбор фактов по транскрипту сессии OpenCode.

Использование:
  node analyze-session.mjs <session.json> [out.txt]

  <session.json>  файл, полученный export-session.mjs (или fix-encoding.mjs)
  [out.txt]       отчёт; по умолчанию <session>.report.txt рядом с JSON

Пример:
  node .opencode/scripts/session-analysis/analyze-session.mjs "$env:TEMP\\opencode\\sessions\\ses_xxx.json"`);
  process.exit(args.length === 0 ? 2 : 0);
}

const SRC = args[0];
const OUT =
  args[1] ||
  join(dirname(SRC), basename(SRC).replace(/\.json$/i, '') + '.report.txt');

let raw;
try {
  raw = readFileSync(SRC, 'utf8');
} catch (e) {
  console.error(`Не читается ${SRC}: ${e.message}`);
  process.exit(2);
}
if (raw.charCodeAt(0) === 0xfeff) raw = raw.slice(1);

let encodingRepaired = false;
const rep = tryRepair(raw);
if (rep.status === 'repaired') {
  raw = rep.candidate;
  encodingRepaired = true;
} else if (rep.status === 'ambiguous') {
  console.error(`Внимание: в ${SRC} возможны следы CP866 (см. fix-encoding.mjs); продолжаю как есть.`);
}

let data;
try {
  data = JSON.parse(raw);
} catch (e) {
  console.error(`JSON не разбирается: ${e.message}`);
  process.exit(1);
}

const L = [];
const push = (x) => L.push(String(x));
const iso = (t) => (t ? new Date(t).toISOString() : '?');
const flat = (x, m = 220) =>
  (typeof x === 'string' ? x : JSON.stringify(x) || '').replace(/\s+/g, ' ').slice(0, m);
const min = (ms) => (ms / 60000).toFixed(1);

const msgs = data.messages || [];
const prompts = [];
msgs.forEach((m, i) => {
  if (m.type === 'user' && typeof m.text === 'string' && m.text.trim()) prompts.push(i);
});

const counts = {};
const stat = {};
const finish = {};
const segStats = [];
const segTools = [];
const segReason = [];
const readPaths = {};
const editPaths = {};
const writePaths = {};
const shellCalls = [];
const executeInputs = [];
const flagged = [];
const cuts = [];
const boundary = [];
const stopTurns = [];
const errorTurns = [];
let prevDone = null;

function segOf(i) {
  let k = -1;
  for (const p of prompts) if (i >= p) k++;
  return k;
}

msgs.forEach((m, i) => {
  const t = m.time || {};
  const gap = prevDone && t.created ? ((t.created - prevDone) / 1000).toFixed(1) : '-';
  prevDone = t.completed || prevDone;

  if (!Array.isArray(m.content)) {
    const kind = m.type || '?';
    push(
      `[${i}] ${kind.toUpperCase()} created=${iso(t.created)} gap=${gap}s` +
        (m.outcome ? ` outcome=${m.outcome}` : '') +
        (kind === 'user' ? ` textLen=${(m.text || '').length}` : '')
    );
    return;
  }

  const k = segOf(i);
  if (!segStats[k]) {
    segStats[k] = { steps: 0, tools: 0, dur: 0, first: null, last: null };
    segTools[k] = {};
    segReason[k] = 0;
  }
  const seg = segStats[k];
  seg.steps++;
  if (t.created) seg.first = seg.first === null ? t.created : seg.first;
  if (t.completed) seg.last = t.completed;
  if (t.created && t.completed) seg.dur += t.completed - t.created;
  finish[m.finish] = (finish[m.finish] || 0) + 1;

  const kinds = {};
  const tools = [];
  let text = '';
  let reason = 0;
  for (const pt of m.content) {
    kinds[pt.type] = (kinds[pt.type] || 0) + 1;
    if (pt.type === 'text') text += pt.text || '';
    else if (pt.type === 'reasoning') {
      reason += (pt.text || '').length;
      segReason[k] += (pt.text || '').length;
    } else if (pt.type === 'tool') {
      const name = pt.name || '?';
      const stt = pt.state || {};
      const st = stt.status || '?';
      tools.push(name + ':' + st);
      counts[name] = (counts[name] || 0) + 1;
      (stat[name] = stat[name] || {})[st] = (stat[name][st] || 0) + 1;
      segTools[k][name] = (segTools[k][name] || 0) + 1;
      seg.tools++;

      const inp = stt.input || {};
      const editPath = inp.path || inp.filePath;
      if (name === 'read' && inp.path) readPaths[inp.path] = (readPaths[inp.path] || 0) + 1;
      if (name === 'edit' && editPath) editPaths[editPath] = (editPaths[editPath] || 0) + 1;
      if (name === 'write' && inp.path) writePaths[inp.path] = (writePaths[inp.path] || 0) + 1;
      if (name === 'execute') executeInputs.push({ i, input: JSON.stringify(inp).slice(0, 300) });

      let outText = '';
      if (Array.isArray(stt.content)) outText = stt.content.map((c) => (c && c.text) || '').join('\n');
      else if (typeof stt.content === 'string') outText = stt.content;
      else if (typeof stt.output === 'string') outText = stt.output;

      if (name === 'shell') {
        shellCalls.push({
          i,
          input: inp.command || JSON.stringify(inp).slice(0, 200),
          status: st,
          error: stt.error ? `${stt.error.type}: ${stt.error.message}` : '',
          out: flat(outText, 200),
        });
      }
      if (outText.includes('[token-guard] срез: опущено')) {
        const at = outText.indexOf('[token-guard] срез: опущено');
        cuts.push({
          i,
          tool: name,
          snip: outText.replace(/\s+/g, ' ').slice(Math.max(0, at - 60), at + 150),
        });
      }
      const marker = /token-guard|rejected|denied|error/i.test(outText.slice(0, 4000));
      if (st !== 'completed' || marker) {
        flagged.push({
          i,
          name,
          st,
          where: inp.path || inp.filePath || inp.command || '',
          error: stt.error ? `${stt.error.type}: ${stt.error.message}` : '',
          out: flat(outText, 300),
        });
      }
    }
  }

  const dur = t.completed && t.created ? ((t.completed - t.created) / 1000).toFixed(1) : '?';
  const empty = !text.trim();
  if (m.finish === 'stop') stopTurns.push({ i, textLen: text.length, head: flat(text, 200) });
  if (m.finish === 'error') errorTurns.push({ i, reasonLen: reason });
  push(
    `[${i}] ASST seg${k} created=${iso(t.created)} dur=${dur}s gap=${gap}s ` +
      `finish=${m.finish || '?'} kinds=${JSON.stringify(kinds)} tools=${tools.join(',')} ` +
      `textLen=${text.length} reasonLen=${reason}` +
      (m.finish === 'stop' && empty ? '  <<< ПУСТОЙ ФИНАЛ' : '')
  );
  const head = flat(text, 160);
  if (head) push('    text: ' + head);

  for (const p of prompts) {
    if (i >= p - 3 && i < p) {
      boundary.push({ i, p, tools: tools.join(','), textLen: text.length, text });
    }
  }
});

// ---------- сборка отчёта ----------

const out = [];
const W = (x) => out.push(String(x));

W('# Отчёт analyze-session.mjs');
W('# Источник: ' + SRC);
W('# Создан: ' + new Date().toISOString());
W('# encoding-repaired: ' + encodingRepaired);
W('');
W('== LEGEND ==');
W('[i]  — индекс сообщения в экспорте (не шаг!); шаг = ассистентское сообщение.');
W('segN — сегмент между промптами; prompt@k — индекс промпта (user с непустым текстом).');
W('finish — финал хода модели: tool-calls | stop | error | ?.');
W('kind=USER/IDLE — промпты и маркеры простоя; ASST — ассистентские ходы.');
W('FLAGGED: permission.rejected = команда отклонена правами (не исполнялась).');
W('token-guard срез = вывод инструмента подрезан плагином (неполный).');
W('<<< ПУСТОЙ ФИНАЛ = ход finish=stop без текста (обрыв/лимит без отчёта).');
W('');

const I = data.info || {};
W('== SESSION ==');
W(`id=${I.id} parent=${I.parentID} agent=${I.agent} outcome=${I.outcome}`);
W(`model=${JSON.stringify(I.model)} cost=${I.cost}`);
W(`tokens=${JSON.stringify(I.tokens)}`);
W(`created=${iso(I.time?.created)} updated=${iso(I.time?.updated)} idle=${iso(I.time?.idle)}`);
W(
  `wall created->updated min=${I.time?.updated ? min(I.time.updated - I.time.created) : '?'} ` +
    `created->idle min=${I.time?.idle ? min(I.time.idle - I.time.created) : '?'}`
);
W('title=' + (I.title || ''));
W('');

W('== PROMPTS ==');
W('prompt indexes=' + prompts.join(','));
W('');

const body = L.join('\n');
W(body);
W('');

W('== SEGMENTS (between prompts) ==');
segStats.forEach((s, k) => {
  if (!s) {
    W(`seg${k}: (пусто)`);
    return;
  }
  W(
    `seg${k} (prompt@${prompts[k]}): steps=${s.steps} tools=${s.tools} reasonChars=${segReason[k]} ` +
      `sumDurMin=${min(s.dur)} wall=${iso(s.first)} -> ${iso(s.last)} tools=${JSON.stringify(segTools[k])}`
  );
});
W('finish distribution: ' + JSON.stringify(finish));
W('ассистентских ходов всего: ' + (msgs.filter((m) => Array.isArray(m.content)).length));
W('');

W('== FINISH / SUSPICIOUS TURNS ==');
W('-- finish=error:');
errorTurns.forEach((t) => W(`[${t.i}] reasonLen=${t.reasonLen} (обрыв потока; ищите авто-продолжение следом)`));
W('-- finish=stop (финалы ходов: отчёты, wrap-up на лимите, пустые финалы):');
stopTurns.forEach((t) =>
  W(`[${t.i}] textLen=${t.textLen} ${t.textLen === 0 ? 'ПУСТОЙ' : 'head: ' + t.head}`)
);
W('');

W('== TOOL TOTALS ==');
W('counts=' + JSON.stringify(counts));
W('status=' + JSON.stringify(stat, null, 1));
W('');

W('== FLAGGED TOOL CALLS ==');
flagged.forEach((f) =>
  W(
    `[${f.i}] ${f.name} status=${f.st}${f.where ? ` where=${f.where}` : ''}` +
      `${f.error ? ` err=${f.error}` : ''}` +
      (f.out ? ` OUT: ${f.out}` : '')
  )
);
W('');

W('== SHELL CALLS ==');
shellCalls.forEach((e) =>
  W(`[${e.i}] status=${e.status}${e.error ? ` err=${e.error}` : ''} :: ${e.input}\n    out: ${e.out}`)
);
W('');

W('== TOKEN-GUARD CUTS ==' + cuts.length);
cuts.forEach((c) => W(`[${c.i}] ${c.tool}: ...${c.snip}...`));
W('');

W('== READS BY PATH ==');
Object.entries(readPaths)
  .sort((a, b) => b[1] - a[1])
  .forEach(([p, c]) => W(`${c}  ${p}`));
W('== EDITS BY PATH ==');
Object.entries(editPaths)
  .sort((a, b) => b[1] - a[1])
  .forEach(([p, c]) => W(`${c}  ${p}`));
W('== WRITES BY PATH ==');
Object.entries(writePaths)
  .sort((a, b) => b[1] - a[1])
  .forEach(([p, c]) => W(`${c}  ${p}`));
W('== EXECUTE INPUTS ==');
executeInputs.forEach((e) => W(`[${e.i}] ${e.input}`));
W('');

W('== USER MESSAGES (full) ==');
msgs.forEach((m, i) => {
  if (m.type !== 'user') return;
  W(`------ [${i}] len=${(m.text || '').length} ------`);
  W(m.text || '');
});
W('');

W('== TAIL TEXTS (последние 8 непустых ассистентских текстов) ==');
let shown = 0;
for (let i = msgs.length - 1; i >= 0 && shown < 8; i--) {
  const m = msgs[i];
  if (!Array.isArray(m.content)) continue;
  const txt = m.content.filter((p) => p.type === 'text').map((p) => p.text || '').join('\n');
  if (!txt.trim()) continue;
  shown++;
  W(`---- [${i}] len=${txt.length} ----`);
  W(txt.slice(0, 3500));
}
W('');

W('== BOUNDARY TEXTS (before each prompt) ==');
boundary.forEach((b) => {
  W(`---- before prompt@${b.p}: [${b.i}] tools=${b.tools} textLen=${b.textLen} ----`);
  W(b.text.slice(0, 2000));
});

writeFileSync(OUT, out.join('\n'), 'utf8');
console.log(`Отчёт: ${OUT} (${out.length} строк)`);
if (encodingRepaired) console.log('Кодировка была испорчена CP866 — текст восстановлен в памяти.');
