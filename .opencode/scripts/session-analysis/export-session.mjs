#!/usr/bin/env node
// export-session.mjs — выгрузка транскрипта сессии OpenCode в JSON без потери
// кодировки.
//
// Почему не `opencode session export … | Out-File`: PowerShell декодирует
// stdout внешней команды как OEM CP866, и русский текст в файле превращается
// в «╨Я╨░╨╝╤П╤В╤М» (лечится fix-encoding.mjs). Этот скрипт забирает stdout
// CLI как буфер байт и пишет их в файл как есть.
//
// Использование:
//   node export-session.mjs <sessionID> [outPath]
//
// По умолчанию: <temp>/opencode/sessions/<sessionID>.json
// Требуется работающий сервис OpenCode (CLI `opencode` в PATH).

import { mkdirSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { tmpdir } from 'node:os';
import { spawnSync } from 'node:child_process';
import { tryRepair } from './fix-encoding.mjs';

function usage() {
  console.log(`Выгрузка транскрипта сессии OpenCode в JSON (UTF-8, без порчи кодировки).

Использование:
  node export-session.mjs <sessionID> [outPath]

Аргументы:
  <sessionID>   идентификатор сессии (ses_…), например из parent-сессии или ленты
  [outPath]     путь к JSON; по умолчанию <temp>/opencode/sessions/<sessionID>.json

Пример (pwsh, из корня репозитория):
  node .opencode/scripts/session-analysis/export-session.mjs ses_f1b2c478bffeP6dGNF2ja8L762

Коды выхода: 0 — ok; 1 — экспорт не похож на JSON; 2 — ошибка запуска/файла.`);
}

const args = process.argv.slice(2);
if (args.length === 0 || args.includes('--help') || args.includes('-h')) {
  usage();
  process.exit(args.length === 0 ? 2 : 0);
}

const session = args[0];
if (!/^ses_[A-Za-z0-9]+$/.test(session)) {
  console.error(`Ошибка: «${session}» не похож на ID сессии (ожидается ses_…).`);
  process.exit(2);
}
const out = args[1] || join(tmpdir(), 'opencode', 'sessions', `${session}.json`);

// Windows: .cmd-шим в PATH виден только через cmd.exe, поэтому команда — одной
// строкой (ID уже проверен регуляркой); остальные ОС — прямой запуск.
const opts = { encoding: 'buffer', maxBuffer: 512 * 1024 * 1024 };
const res =
  process.platform === 'win32'
    ? spawnSync(`opencode session export ${session}`, { ...opts, shell: true })
    : spawnSync('opencode', ['session', 'export', session], opts);

if (res.error) {
  console.error(`Не удалось запустить «opencode»: ${res.error.message}`);
  process.exit(2);
}
if (res.status !== 0) {
  const err = (res.stderr || Buffer.alloc(0)).toString('utf8').trim();
  console.error(
    `opencode session export завершился с кодом ${res.status}` +
      (err ? `:\n${err}` : '. Проверьте ID сессии и что сервис запущен.')
  );
  process.exit(res.status || 2);
}

let text = (res.stdout || Buffer.alloc(0)).toString('utf8');
let note = '';
if (text.charCodeAt(0) === 0xfeff) text = text.slice(1);

let data;
try {
  data = JSON.parse(text);
} catch (e) {
  console.error(
    `Экспорт не является JSON (${e.message}). Начало вывода: ${JSON.stringify(text.slice(0, 120))}`
  );
  process.exit(1);
}

// Страховка: если кодировку всё-таки испортило окружение (CLI/шелл) — чиним.
const rep = tryRepair(text);
if (rep.status === 'repaired') {
  text = rep.candidate;
  try {
    data = JSON.parse(text);
    note = ' · кодировка была испорчена CP866 — починена';
  } catch {
    /* оставляем как есть; ниже сообщим */
  }
} else if (rep.status === 'ambiguous') {
  note = ' · внимание: возможны следы CP866, проверьте fix-encoding.mjs';
}

mkdirSync(dirname(out), { recursive: true });
writeFileSync(out, text, 'utf8');

const messages = Array.isArray(data.messages) ? data.messages.length : '?';
console.log(`Экспорт: ${out}${note}`);
console.log(
  `Сессия: ${data.info?.id || session} · агент: ${data.info?.agent || '?'} · ` +
    `сообщений: ${messages} · размер: ${Buffer.byteLength(text)} Б`
);
