#!/usr/bin/env node
// fix-encoding.mjs — восстановление UTF-8 текста, испорченного перекодировкой
// через CP866. Типовой случай: `opencode session export | Out-File` в Windows
// PowerShell — русский текст превращается в «╨Я╨░╨╝╤П╤В╤М» (UTF-8 байты,
// декодированные как OEM CP866 и заново записанные в UTF-8).
//
// Библиотека для других скриптов:
//   repairMojibake(text)  → строка, восстановленная из CP866-кракозябр;
//   tryRepair(text)       → { status: 'clean' | 'repaired' | 'ambiguous', candidate? }.
//
// CLI:
//   node fix-encoding.mjs <file> [--out <file>] [--in-place] [--force]
//   node fix-encoding.mjs --selftest
//
// Коды выхода: 0 — чисто/починено; 1 — неоднозначно, файл не изменён; 2 — ошибка.

import { readFileSync, writeFileSync } from 'node:fs';
import { pathToFileURL } from 'node:url';

const BOX_RE = /[\u2500-\u257f]/g; // рамки — следы байтов UTF-8 в CP866
const REPLACEMENT_RE = /\uFFFD/g; // «потерянные» байты после обратного маппинга

function getIbm866Decoder() {
  try {
    return new TextDecoder('ibm866');
  } catch {
    throw new Error(
      'в этой сборке Node нет декодера ibm866 (нужен полный ICU). ' +
        'Проверьте: node -e "console.log(new TextDecoder(\'ibm866\').decode(Uint8Array.of(0xd0)))"'
    );
  }
}

let reverseTable = null;

/** Обратная таблица CP866: символ → байт (все 256 байтов биективны). */
function getReverseTable() {
  if (reverseTable) return reverseTable;
  const dec = getIbm866Decoder();
  reverseTable = new Map();
  for (let b = 0; b < 256; b++) {
    reverseTable.set(dec.decode(Uint8Array.of(b)), b);
  }
  return reverseTable;
}

/** Восстанавливает текст, испорченный CP866-декодированием UTF-8. */
export function repairMojibake(text) {
  const table = getReverseTable();
  const buf = Buffer.alloc(text.length * 4);
  let n = 0;
  for (const ch of text) {
    const b = table.get(ch);
    if (b !== undefined) {
      buf[n++] = b; // символ CP866 → исходный байт
    } else {
      const t = Buffer.from(ch, 'utf8'); // прочие символы — как есть
      t.copy(buf, n);
      n += t.length;
    }
  }
  return buf.subarray(0, n).toString('utf8');
}

/** Число «рамочных» символов в тексте. */
export function countBox(text) {
  return (text.match(BOX_RE) || []).length;
}

/**
 * Пытается починить текст. Ничего не пишет на диск.
 * @returns {{status:'clean'}|{status:'repaired',candidate:string}|{status:'ambiguous',candidate:string}}
 */
export function tryRepair(text) {
  if (countBox(text) === 0) return { status: 'clean' };
  const candidate = repairMojibake(text);
  const bad = (candidate.match(REPLACEMENT_RE) || []).length;
  if (bad > 0) return { status: 'ambiguous', candidate };
  // JSON проверяем разбором — гарантия, что структура не разрушена.
  const t = text.trimStart();
  if (t.startsWith('{') || t.startsWith('[')) {
    try {
      JSON.parse(candidate);
    } catch {
      return { status: 'ambiguous', candidate };
    }
  }
  return { status: 'repaired', candidate };
}

function usage() {
  console.log(`Починка CP866-кракозябр в текстовых (обычно JSON) файлах.

Использование:
  node fix-encoding.mjs <file> [опции]
  node fix-encoding.mjs --selftest

Опции:
  --out <file>   куда записать результат (по умолчанию <file> → <file>.fixed.json)
  --in-place     перезаписать исходный файл
  --force        писать, даже если результат неоднозначен (есть «�»)
  --selftest     проверить работу маппинга на встроенном примере
  --help, -h     эта справка

Результат (stdout): строка «Чисто: …» либо «Починено: … → …».
Коды выхода: 0 — чисто/починено; 1 — неоднозначно; 2 — ошибка вызова/файла.`);
}

function selftest() {
  const sample =
    'Правило МинимальныйВозраст: «Отказ» — Память роли, ↯ нет ошибок.';
  const dec = getIbm866Decoder();
  const broken = dec.decode(Buffer.from(sample, 'utf8')); // симулируем порчу
  const repaired = repairMojibake(broken);
  const pass =
    repaired === sample &&
    tryRepair(broken).status === 'repaired' &&
    tryRepair(sample).status === 'clean';
  console.log(`selftest: ${pass ? 'ok' : 'ПРОВАЛ'}`);
  console.log(`  sample : ${JSON.stringify(sample)}`);
  console.log(`  broken : ${JSON.stringify(broken)}`);
  console.log(`  repair : ${JSON.stringify(repaired)}`);
  return pass ? 0 : 1;
}

function main(argv) {
  const args = argv.slice(2);
  if (args.includes('--selftest')) return selftest();
  if (args.length === 0 || args.includes('--help') || args.includes('-h')) {
    usage();
    return args.length === 0 ? 2 : 0;
  }

  const file = args.find((a) => !a.startsWith('-'));
  if (!file) {
    usage();
    return 2;
  }
  const inPlace = args.includes('--in-place');
  const force = args.includes('--force');
  const outIdx = args.indexOf('--out');
  const outFlag = outIdx >= 0 ? args[outIdx + 1] : undefined;
  if (outIdx >= 0 && (!outFlag || outFlag.startsWith('-'))) {
    console.error('Ошибка: после --out нужен путь к файлу.');
    return 2;
  }

  let raw;
  try {
    raw = readFileSync(file, 'utf8');
  } catch (e) {
    console.error(`Не читается ${file}: ${e.message}`);
    return 2;
  }
  const hadBom = raw.charCodeAt(0) === 0xfeff;
  if (hadBom) raw = raw.slice(1);

  const res = tryRepair(raw);
  if (res.status === 'clean') {
    console.log(
      `Чисто: ${file} — следов CP866 нет, файл не изменён` +
        (hadBom ? ' (BOM обнаружен и может мешать JSON.parse).' : '.')
    );
    return 0;
  }
  if (res.status === 'ambiguous' && !force) {
    console.error(
      `Неоднозначно: ${file} — после обратного маппинга остаются «�»; ` +
        'файл не изменён. Если уверены в порче — повторите с --force.'
    );
    return 1;
  }

  const defaultOut = (file.endsWith('.json') ? file.slice(0, -5) : file) + '.fixed.json';
  const target = inPlace ? file : outFlag || defaultOut;
  try {
    writeFileSync(target, res.candidate, 'utf8');
  } catch (e) {
    console.error(`Не записывается ${target}: ${e.message}`);
    return 2;
  }
  console.log(`${res.status === 'repaired' ? 'Починено' : 'Записано (--force)'}: ${file} → ${target}`);
  return 0;
}

const isMain =
  process.argv[1] && pathToFileURL(process.argv[1]).href === import.meta.url;
if (isMain) process.exit(main(process.argv));
