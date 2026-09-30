#!/usr/bin/env node
// clean-logs.mjs — быстрая очистка операционных логов агентов CREDO.
//
// Что делает:
//   memory — каждый .opencode/memory/<роль>.md возвращается к шаблону:
//            шапка до заголовка «## Чекпойнты» + «- Чекпойнтов ещё не было.»;
//   mail   — удаляются ленты задач .opencode/mail/*.md (.gitkeep остаётся);
//            перед удалением — best-effort снимок событий лент в архив pm
//            (`uv run pm-agents --no-viz`), чтобы история не терялась.
//
// Использование (из любого каталога):
//   node .opencode/scripts/clean-logs.mjs [--dry-run] [--no-backup] [--no-snapshot]
//                                         [--memory-only | --mail-only]
//
// По умолчанию перед изменениями делается бэкап затрагиваемых файлов в
// <temp>/opencode/logs-backup-<timestamp>; сбой снимка pm — предупреждение,
// очистка продолжается.

import { spawnSync } from 'node:child_process';
import {
  copyFileSync,
  existsSync,
  mkdirSync,
  readFileSync,
  readdirSync,
  rmSync,
  writeFileSync,
} from 'node:fs';
import { basename, dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { tmpdir } from 'node:os';

const HEADING = '## Чекпойнты';
const PLACEHOLDER = '- Чекпойнтов ещё не было.';

const args = new Set(process.argv.slice(2));
if (args.has('--help') || args.has('-h')) {
  printUsage();
  process.exit(0);
}

const dryRun = args.has('--dry-run');
const noBackup = args.has('--no-backup');
const noSnapshot = args.has('--no-snapshot');
const memoryOnly = args.has('--memory-only');
const mailOnly = args.has('--mail-only');

if (memoryOnly && mailOnly) {
  console.error('Ошибка: укажите что-то одно — --memory-only или --mail-only.');
  process.exit(2);
}

// .opencode/scripts/ → .opencode
const opencodeDir = join(dirname(fileURLToPath(import.meta.url)), '..');
const memoryDir = join(opencodeDir, 'memory');
const mailDir = join(opencodeDir, 'mail');

function printUsage() {
  console.log(`Очистка операционных логов агентов CREDO (memory + mail).

Использование:
  node .opencode/scripts/clean-logs.mjs [опции]

Опции:
  --dry-run        показать план, ничего не менять
  --no-backup      не делать бэкап (по умолчанию — делается)
  --no-snapshot    не делать снимок лент в архив pm (по умолчанию — делается)
  --memory-only    только память ролей
  --mail-only      только ленты задач
  --help, -h       эта справка

Бэкап: <temp>/opencode/logs-backup-<YYYYMMDD-HHmmss>/
Память: .opencode/memory/<роль>.md → шаблон с «${PLACEHOLDER}»
Почта:  .opencode/mail/*.md → удаляются (.gitkeep остаётся)
Перед удалением лент — best-effort снимок событий в архив pm
(uv run pm-agents --no-viz в .opencode/scripts/pm); сбой снимка —
предупреждение, очистка продолжается. Отключить: --no-snapshot.`);
}

function timestamp() {
  const d = new Date();
  const p = (n) => String(n).padStart(2, '0');
  return (
    `${d.getFullYear()}${p(d.getMonth() + 1)}${p(d.getDate())}` +
    `-${p(d.getHours())}${p(d.getMinutes())}${p(d.getSeconds())}`
  );
}

/** План для памяти: reset (в шаблон) / clean (уже чистый) / skip (нет заголовка). */
function planMemory() {
  if (!existsSync(memoryDir)) return [];
  const plan = [];
  for (const name of readdirSync(memoryDir).filter((f) => f.endsWith('.md')).sort()) {
    const path = join(memoryDir, name);
    const text = readFileSync(path, 'utf8');
    const eol = text.includes('\r\n') ? '\r\n' : '\n';
    const lines = text.split(/\r?\n/);
    const idx = lines.findIndex((l) => l.trimEnd() === HEADING);
    if (idx < 0) {
      plan.push({ path, name, kind: 'skip' });
      continue;
    }
    const tail = lines.slice(idx + 1).filter((l) => l.trim() !== '');
    const alreadyClean =
      tail.length === 0 || (tail.length === 1 && tail[0].trim() === PLACEHOLDER);
    if (alreadyClean) {
      plan.push({ path, name, kind: 'clean' });
      continue;
    }
    const content = lines.slice(0, idx + 1).join(eol) + eol + eol + PLACEHOLDER + eol;
    plan.push({ path, name, kind: 'reset', content });
  }
  return plan;
}

/** План для почты: все *.md в корне .opencode/mail. */
function planMail() {
  if (!existsSync(mailDir)) return [];
  return readdirSync(mailDir)
    .filter((f) => f.endsWith('.md') && f !== '.gitkeep')
    .sort()
    .map((name) => ({ path: join(mailDir, name), name, kind: 'delete' }));
}

/** Best-effort снимок событий лент в накопительный архив pm. */
function snapshotPm() {
  const pmDir = join(opencodeDir, 'scripts', 'pm');
  if (!existsSync(join(pmDir, 'pyproject.toml'))) {
    console.warn('pm: проект .opencode/scripts/pm не найден — снимок пропущен.');
    return;
  }
  console.log('pm: снимок событий в архив (uv run pm-agents --no-viz)…');
  const result = spawnSync('uv', ['run', 'pm-agents', '--no-viz'], {
    cwd: pmDir,
    stdio: 'inherit',
    timeout: 300000,
  });
  if (result.error || result.status !== 0) {
    const reason = result.error ? result.error.message : `код ${result.status}`;
    console.warn(
      `pm: снимок не удался (${reason}); ленты будут удалены без пополнения архива. ` +
        'Повторить: uv run pm-agents; отключить: --no-snapshot.'
    );
  } else {
    console.log('pm: снимок в архив выполнен.');
  }
}

function makeBackup(files) {
  const dir = join(tmpdir(), 'opencode', `logs-backup-${timestamp()}`);
  for (const file of files) {
    const sub = file.path.startsWith(memoryDir) ? 'memory' : 'mail';
    mkdirSync(join(dir, sub), { recursive: true });
    copyFileSync(file.path, join(dir, sub, basename(file.path)));
  }
  return dir;
}

function main() {
  const memoryPlan = mailOnly ? [] : planMemory();
  const mailPlan = memoryOnly ? [] : planMail();

  const toReset = memoryPlan.filter((f) => f.kind === 'reset');
  const toDelete = mailPlan.filter((f) => f.kind === 'delete');
  const skipped = memoryPlan.filter((f) => f.kind === 'skip');
  const clean = memoryPlan.filter((f) => f.kind === 'clean');

  console.log(
    `${dryRun ? '[dry-run] ' : ''}memory: сбросить ${toReset.length}, уже чистых ${clean.length}` +
      (skipped.length ? `, без заголовка ${skipped.length}` : '') +
      `; mail: удалить ${toDelete.length}`
  );
  for (const f of toReset) console.log(`  memory: сброс  ${f.name}`);
  for (const f of skipped) console.log(`  memory: ПРОПУСК ${f.name} (нет «${HEADING}»)`);
  for (const f of toDelete) console.log(`  mail:   удалить ${f.name}`);

  if (toReset.length === 0 && toDelete.length === 0) {
    console.log('Нечего очищать.');
    return;
  }

  if (dryRun) {
    if (!noSnapshot && toDelete.length > 0) {
      console.log(
        '[dry-run] pm: был бы снимок лент в архив (uv run pm-agents --no-viz).'
      );
    }
    console.log('Ничего не изменено (--dry-run).');
    return;
  }

  if (!noBackup) {
    const dir = makeBackup([...toReset, ...toDelete]);
    console.log(`Бэкап: ${dir}`);
  }

  if (!noSnapshot && toDelete.length > 0) snapshotPm();

  for (const f of toReset) writeFileSync(f.path, f.content, 'utf8');
  for (const f of toDelete) rmSync(f.path);

  console.log(
    `Готово: память — ${toReset.length}, почта — ${toDelete.length}.` +
      (skipped.length ? ` Пропущено файлов без заголовка: ${skipped.length}.` : '')
  );
}

main();
