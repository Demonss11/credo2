# session-analysis — разбор сессий агентов

Инструменты постфактум-разбора сессий субагентов (после прогонов ролей):
за один проход — от **корневой сессии прогона** до **черновика служебного
отчёта по конкретному агенту**.

**Кто запускает.** Сервисная сессия владельца (или сам владелец). Нужны `node`
(с полным ICU) и CLI `opencode`. У ролей рабочей группы `node` нет в allowlist
shell — это не их зона; роли только читают и цитируют уже готовые отчёты.

**Что считается «хорошим» результатом.** Служебный отчёт-разбор в
`docs/analysis/` (примеры: `T-04-run5-coder-session.md` …
`T-04-run5-git-session.md`) + при необходимости — кандидат F-записи для
`docs/analysis/findings-registry.md` (вносит `migrator`) и уточнение для
меморандума прогона (вносит сервисная сессия).

## Полный цикл (рекомендуется)

Вход — **корневая сессия + агент**; выход — артефакты в
`%TEMP%\opencode\sessions\`:

```powershell
node .opencode/scripts/session-analysis/agent-report.mjs ses_Корень tester
#   ses_<ребёнок>.json        — транскрипт (export-session.mjs)
#   ses_<ребёнок>.report.txt  — факты (analyze-session.mjs)
#   ses_<ребёнок>.draft.md    — черновик отчёта: факты заполнены, анализ — по шаблону

# Посмотреть сессии агента и взять только последнюю:
node .opencode/scripts/session-analysis/agent-report.mjs ses_Корень git --list
node .opencode/scripts/session-analysis/agent-report.mjs ses_Корень git --last
```

Дальше: дополнить `*.draft.md` по шаблону
[`docs/analysis/session-report-template.md`](../../../docs/analysis/session-report-template.md)
и сохранить как `docs/analysis/<T-XX>-run<N>-<роль>-session.md`.

Опции: `--last` (только последняя сессия агента), `--list` (список без
отчётов), `--fresh` (переэкспорт), `--facts-only`, `--out <dir>`, `--json`.
Если у агента несколько сессий (resume, повторные пакеты) — по умолчанию
обрабатываются все.

## По шагам (ручной контроль)

```powershell
# 0. ID сессий: top-level — `opencode session list`; субагентские — из корневой
#    (find-subagents.mjs; см. раздел ниже):
node .opencode/scripts/session-analysis/find-subagents.mjs ses_Корень
node .opencode/scripts/session-analysis/find-subagents.mjs ses_Корень --export

# 1. Экспорт транскрипта (UTF-8, без порчи кодировки):
node .opencode/scripts/session-analysis/export-session.mjs ses_XXXXXXXX
#    → %TEMP%\opencode\sessions\ses_XXXXXXXX.json

# 2. Сбор фактов:
node .opencode/scripts/session-analysis/analyze-session.mjs "$env:TEMP\opencode\sessions\ses_XXXXXXXX.json"
#    → %TEMP%\opencode\sessions\ses_XXXXXXXX.report.txt

# 3. Только если JSON пришёл из старой выгрузки (см. «Грабли»):
node .opencode/scripts/session-analysis/fix-encoding.mjs путь\к\файлу.json --out путь\к\чистому.json

# 4. Собрать служебный отчёт по шаблону:
#    docs/analysis/session-report-template.md → docs/analysis/<T-XX>-run<N>-<роль>-session.md
```

Артефакты — **всегда вне репозитория** (temp-каталог); в git попадает только
итоговый отчёт `docs/analysis/`.

## Как выйти из корневой сессии в субагентские

`opencode session list` знает только top-level сессии; ID дочерних живут в
транскрипте родителя: результат каждого вызова инструмента `subagent` начинается
строкой `<subagent sessionID="ses_…" state="…">`. Скрипт читает этот маркер сам:

```powershell
node .opencode/scripts/session-analysis/find-subagents.mjs ses_f1b8398d2ffe4VC7LhwgnYggwl
# Сессия: … · агент: build · родитель: —
# Вызовов subagent: 19 · уникальных дочерних сессий: 16
# #  сессия                          агент        статус     msg     описание
# 1  ses_f1b622b34ffe3pt5qaroUrbVvB  git          completed  [143]   Коммит пакета w0_i3_i4
# 7  ses_f1b4a5adaffeKrvZFV3dxx9dZd  coder        completed  [176]   T-04: конверт ошибок MCP  (×2)
# …

node …find-subagents.mjs ses_Корень --json     # машинный вывод (session, parentID, calls, children[])
node …find-subagents.mjs ses_Корень --fresh    # переэкспортировать родителя
node …find-subagents.mjs ses_Корень --export   # плюс выгрузить JSON всех детей
```

- Повторы вызова одной роли (resume тем же `sessionID`) видны как `(×N)` — у
  дочерней сессии один ID, вызовов может быть несколько.
- Если передать транскрипт **дочерней** сессии, скрипт предупредит и покажет
  `parentID` (в экспорте ребёнка есть `info.parentID`); список сиблингов
  запрашивайте на родителе.
- Родитель без вызовов `subagent` → «дочерних сессий не найдено» (не ошибка).
- Работает и от готового JSON: `node find-subagents.mjs parent.json`.

## Файлы

| Файл | Назначение | Вход → выход |
|---|---|---|
| `agent-report.mjs` | **полный цикл**: корневая сессия + агент → транскрипт, факты и черновик отчёта | `<ses_корень \| parent.json> <агент> [--last\|--list\|--fresh\|--facts-only\|--out <dir>\|--json]` → файлы в `<temp>/opencode/sessions/` |
| `export-session.mjs` | выгрузка транскрипта: stdout CLI забирается как буфер байт, без декодирования PowerShell | `ses_… [out.json]` → JSON (по умолчанию `<temp>/opencode/sessions/<ses>.json`) |
| `find-subagents.mjs` | переход корень → субагентские: список детей из вызовов `subagent` в транскрипте родителя | `ses_… \| parent.json [--json] [--fresh] [--export]` → таблица/JSON; `--export` — ещё и JSON детей |
| `fix-encoding.mjs` | починка/диагностика CP866-кракозябр; библиотека `repairMojibake`/`tryRepair` для других скриптов | `<file> [--out\|--in-place] [--force] [--selftest]` |
| `analyze-session.mjs` | сбор фактов по транскрипту в текстовый отчёт (секции ниже) | `<session.json> [out.txt]` → `<session>.report.txt` |
| `README.md` | этот файл | — |

Проверка инструментов:

```powershell
node .opencode/scripts/session-analysis/fix-encoding.mjs --selftest   # expect: selftest: ok
node .opencode/scripts/session-analysis/agent-report.mjs --help
```

## Что содержит отчёт `analyze-session.mjs`

Порядок секций:

1. `LEGEND` — как читать строки отчёта.
2. `SESSION` — id, parent, агент, модель, outcome, cost, токены, времена, wall.
3. `PROMPTS` + `MESSAGES` — все сообщения: `[i] ASST segK created/dur/gap
   finish kinds tools textLen reasonLen` + заголовок текста; `USER`/`IDLE`.
4. `SEGMENTS` — по сегментам (между промптами): число шагов, инструменты,
   символы reasoning, суммарная длительность, окно времени.
5. `FINISH / SUSPICIOUS TURNS` — распределение `finish`, все `error`-ходы,
   все `stop`-финалы (в т.ч. пустые — «ПУСТОЙ ФИНАЛ»).
6. `TOOL TOTALS` — счётчики и статусы по инструментам.
7. `FLAGGED TOOL CALLS` — статус ≠ completed и маркеры (`permission.rejected`,
   `token-guard`, `error`).
8. `SHELL CALLS` — все shell-вызовы с ошибками и хвостом вывода.
9. `TOKEN-GUARD CUTS` — какие выводы подрезаны плагином (неполные).
10. `READS / EDITS / WRITES BY PATH` — по путям, с частотами.
11. `EXECUTE INPUTS` — вызовы Code Mode (например, rust-analyzer).
12. `USER MESSAGES` — полные тексты промптов (включая resume-инструкции).
13. `TAIL TEXTS` — последние непустые тексты ассистента (обычно итоговый отчёт).
14. `BOUNDARY TEXTS` — тексты ходов перед каждым промптом (виден обрыв).

Черновик полного цикла (`*.draft.md`) заполняет по этим фактам шапку и §1–§3
(сводка, сегменты, отказы/срезы); §4–§8 — заготовки под анализ.

## Шпаргалка: что искать

| Признак в отчёте | Что означает |
|---|---|
| сегмент ровно N шагов = `steps` роли из `.opencode/agents/<роль>.md` | упор в лимит шагов; смотреть `stop`-финал сегмента |
| `finish=stop`, большой `textLen`, текст «достигнут лимит/НЕ готово» | wrap-up на лимите (отчётность сохранилась) |
| `finish=stop`, `textLen=0` («ПУСТОЙ ФИНАЛ») | обрыв без отчёта — потеря отчётности |
| `finish=error` | обрыв потока; часто следом синтетическое продолжение (EN-промпт «The previous response was interrupted…») |
| `permission.rejected` | команда отклонена правами и **не исполнялась**; сверить с allowlist роли |
| `token-guard срез` | вывод инструмента подрезан (>12 КБ) — неполный |
| сегмент начался с правок ленты/памяти | resume по протоколу «сначала записи» (хороший паттерн) |
| shell-команда с `;` или `\|` | составная — движок прав отклоняет целиком |

## Шаблон служебного отчёта

Имя файла: `docs/analysis/<T-XX>-run<N>-<роль>-session.md`; примеры —
`T-04-run5-coder-session.md` … `T-04-run5-git-session.md` (пять сессий Run 5).

Полный шаблон — инструкция, скелет для копирования, чек-лист и типовые ошибки:
[`docs/analysis/session-report-template.md`](../../../docs/analysis/session-report-template.md).
Перед сдачей отчёта обязательно пройти его чек-лист. Черновик полного цикла
(`*.draft.md`) уже содержит шапку и сводку по этому шаблону — остаётся
дописать анализ (§4–§8) и перенести файл в `docs/analysis/`.

## Грабли

- **CP866-«кракозябры».** `opencode session export … | Out-File` в Windows
  PowerShell декодирует UTF-8 как OEM CP866: `Память` → `╨Я╨░╨╝╤П╤В╤М`.
  Поэтому выгрузка — только `export-session.mjs`. Старые испорченные файлы
  лечит `fix-encoding.mjs` (идемпотентен: чистый файл не трогает).
- **У агента может быть несколько сессий** (resume, повторные пакеты: у `git`
  в Run 5 — 5: `w0_i3_i4`, `w0_merge`, `branch_start`, `branch_end`,
  `run5_records`). Полный цикл по умолчанию обрабатывает все; `--last` — только
  последнюю, `--list` — посмотреть перед выбором.
- **Черновик ≠ отчёт.** `*.draft.md` — заготовка с машинными фактами; оценка,
  выводы и предложения дописываются и проверяются по чек-листу шаблона.
- **BOM.** `Out-File -Encoding utf8` добавляет BOM; скрипты его срезают сами,
  но сторонние JSON-парсеры могут спотыкаться — не используйте Out-File.
- **Индексы `[i]` — не шаги.** Шаг — ассистентское сообщение; в отчёте они
  нумеруются индексом сообщения в экспорте.
- **Отчёт большой** (150–400 строк) — читать страницами по 100–200 строк.
- **ID субагентских сессий нет** в `opencode session list` (он про top-level);
  брать из parent-сессии (`find-subagents.mjs` — маркер `<subagent sessionID=…>`),
  ленты, `progress.yaml` или UI.
- **`session export` требует запущенного сервиса** OpenCode.
- **Не писать артефакты разбора в репозиторий** — только `%TEMP%\opencode\` и
  итоговый отчёт `docs/analysis/`.
- **Файлы — сервисная зона.** Правки этих скриптов — через сервисную сессию
  владельца (как и остальной `.opencode/scripts/`).
