//! D64 (Q60) + D77 (Q73) + D80 (Q76) + D82 (Q78): целостность журнала Q/D.
//!
//! Машинный гейт документации журнала — образец [`tests/features_inventory.rs`].
//! Тест не дублирует прогон продукта: он проверяет структуру и согласованность
//! канон-артефактов журнала (вопросы, решения, каталоги, `TRACEABILITY`,
//! задачи, фичи) и запреты политики ссылок [D65](Q61).
//!
//! v0.1-проверки (карточка `docs/tasks/T-18-docs-journal-test/README.md`):
//! 1. уникальность и целостность ID `Q`/`D` (непрерывная нумерация без дублей);
//! 2. парность `Q`↔`D` (у `Q` — решение либо `dropped`/«закрыт попутно»/`open`;
//!    у `D` — существующий `Q`, кроме ретро-D `D1`–`D5`, `D7`–`D11`, `D13` — D69);
//! 3. строки-представления: `D` — в `decisions/README.md`; `Q` — в
//!    `questions/README.md` и `TRACEABILITY.md`;
//! 4. вердикт «Сверка с кодом» в каждом `D`;
//! 5. согласованность `TRACEABILITY`: словарь `{open, in work, done}`,
//!    `resolved` не допускается (D82), `in work` ⇒ открытая задача,
//!    `done`/`open` ⇒ открытых задач нет;
//! 6. полнота задач (D77): все `T-XX` реестра видны в `TRACEABILITY` и обратно;
//!    статусы ⬜/🚧/✅ совпадают;
//! 7. полнота фич (D80): каждая `*.feature` поимённо в колонке «Реализация»;
//!    wildcard-обобщения не допускаются; обратная живость;
//! 8. запреты (D65): номера строк, миграционные маркеры, ссылки на
//!    удаляемые/сессионные данные (whitelist — `findings-registry.md`, D48).
//!
//! Границы (карточка T-18): проверки вне исторических зон `docs/reviews/**` и
//! `docs/analysis/**` (там допустимы снимки — старые адреса и номера строк);
//! полный link-check относительных ссылок — v0.2. Секции «Сверка с кодом» в
//! `D`-файлах — датированный снимок (аналог исторических зон), поэтому номера
//! строк в них не гейтятся; запрет номеров строк действует на «живую»
//! навигацию (шапки, каталоги, таблицы `TRACEABILITY`).

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

/// Корень репозитория (крейт).
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn docs_dir() -> PathBuf {
    root().join("docs")
}

fn read(path: &Path) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("не читается {}: {e}", path.display()))
}

// ---------------------------------------------------------------------------
// Разбор артефактов журнала
// ---------------------------------------------------------------------------

/// Файлы `Qn.md` / `Dn-<слаг>.md`: номер → путь. Номер уникален (дубль — паника).
fn numbered_files(dir: &Path, prefix: char) -> BTreeMap<u32, PathBuf> {
    let mut map = BTreeMap::new();
    let entries = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("нет каталога {}: {e}", dir.display()));
    for entry in entries {
        let path = entry.expect("не читается запись каталога").path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        let Some(rest) = name.strip_prefix(prefix) else {
            continue;
        };
        let digits: String =
            rest.chars().take_while(|c| c.is_ascii_digit()).collect();
        if digits.is_empty() {
            continue;
        }
        // Q: строго `<digits>.md`; D: `<digits>-<слаг>.md`.
        let tail = &rest[digits.len()..];
        let shaped = if prefix == 'Q' {
            tail == ".md"
        } else {
            tail.starts_with('-') && tail.ends_with(".md")
        };
        assert!(
            shaped,
            "{name}: имя не соответствует шаблону `{prefix}<N>{{...}}.md`"
        );
        let num: u32 = digits
            .parse()
            .unwrap_or_else(|_| panic!("{name}: номер не парсится"));
        assert!(
            map.insert(num, path.clone()).is_none(),
            "дубликат номера {prefix}{num}: {}",
            path.display()
        );
    }
    map
}

/// Требует непрерывную нумерацию `1..=max` (и непустой набор).
fn assert_contiguous(nums: &BTreeSet<u32>, what: &str) {
    assert!(!nums.is_empty(), "{what}: не найдено ни одного файла");
    let max = *nums.iter().next_back().unwrap();
    let expected: BTreeSet<u32> = (1..=max).collect();
    let missing: Vec<u32> = expected.difference(nums).copied().collect();
    assert!(
        missing.is_empty(),
        "{what}: пропуски номеров (непрерывность 1..={max}): {missing:?}"
    );
}

/// Значение поля шапки `- **Field:**` (текст после маркера на той же строке,
/// без ведущих/хвостовых пробелов).
fn field_line<'a>(text: &'a str, field: &str) -> Option<&'a str> {
    let marker = format!("- **{field}:**");
    text.lines().find_map(|line| {
        line.trim_start()
            .strip_prefix(marker.as_str())
            .map(str::trim)
    })
}

/// Все ссылки `Q<n>` / `D<n>` в тексте (границы — не-алфанумерика).
fn find_refs(text: &str, prefix: char) -> BTreeSet<u32> {
    let chars: Vec<char> = text.chars().collect();
    let mut out = BTreeSet::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == prefix {
            let prev_ok = i == 0 || !chars[i - 1].is_alphanumeric();
            if prev_ok {
                let mut j = i + 1;
                // У задач ID — `T-XX` (дефис после префикса).
                if j < chars.len() && chars[j] == '-' {
                    j += 1;
                }
                let digits_start = j;
                while j < chars.len() && chars[j].is_ascii_digit() {
                    j += 1;
                }
                let next_ok = j == chars.len() || !chars[j].is_alphanumeric();
                if j > digits_start && next_ok {
                    let s: String = chars[digits_start..j].iter().collect();
                    out.insert(s.parse().unwrap());
                }
            }
        }
        i += 1;
    }
    out
}

/// Текст секции `## <heading>` до следующего `## `. Заголовок ищется только
/// в начале строки (inline-упоминание `## <heading>` не считается — как в
/// `live_text`).
fn section<'a>(text: &'a str, heading: &str) -> Option<&'a str> {
    let marker = format!("## {heading}");
    let start = if text.starts_with(&marker) {
        0
    } else {
        text.find(&format!("\n{marker}"))? + 1
    };
    let after = &text[start + marker.len()..];
    let end = after.find("\n## ").map(|p| p + 1).unwrap_or(after.len());
    Some(&after[..end])
}

/// Разбивает markdown-строку таблицы на ячейки.
fn table_cells(line: &str) -> Vec<&str> {
    line.trim()
        .trim_matches('|')
        .split('|')
        .map(str::trim)
        .collect()
}

// --- TRACEABILITY -----------------------------------------------------------

#[derive(Debug)]
struct TraceRow {
    q: Option<u32>,
    d: Option<u32>,
    lifecycle: String,
    tasks: String,
    realization: String,
}

fn traceability_rows(text: &str) -> Vec<TraceRow> {
    let mut rows = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if !line.starts_with("| [Q") {
            continue;
        }
        let cells = table_cells(line);
        assert!(
            cells.len() >= 5,
            "TRACEABILITY: строка короче таблицы: {line}"
        );
        rows.push(TraceRow {
            q: find_refs(cells[0], 'Q').into_iter().next(),
            d: find_refs(cells[1], 'D').into_iter().next(),
            lifecycle: cells[2].to_string(),
            tasks: cells[3].to_string(),
            realization: cells[4].to_string(),
        });
    }
    assert!(
        !rows.is_empty(),
        "TRACEABILITY: не найдено ни одной строки таблицы"
    );
    rows
}

fn is_open_task_status(ch: char) -> bool {
    matches!(ch, '⬜' | '🚧')
}

/// Статусы задач из ячейки «Задачи»: `T-XX` → множество пометок ⬜/🚧/✅.
fn cell_task_statuses(cell: &str) -> BTreeMap<u32, BTreeSet<char>> {
    let chars: Vec<char> = cell.chars().collect();
    let mut out: BTreeMap<u32, BTreeSet<char>> = BTreeMap::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == 'T' && i > 0 && chars[i - 1] == '[' {
            let mut j = i + 1;
            // ID задачи — `T-XX` (дефис после префикса, как в `find_refs`).
            if j < chars.len() && chars[j] == '-' {
                j += 1;
            }
            let digits_start = j;
            while j < chars.len() && chars[j].is_ascii_digit() {
                j += 1;
            }
            if j > digits_start {
                let num: u32 = chars[digits_start..j]
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap();
                // Статус — первый ⬜/🚧/✅ после markdown-ссылки `[…](…)`,
                // по всей ячейке (не окном фиксированной длины).
                let mut k = j;
                if chars.get(k) == Some(&']')
                    && let Some(close) =
                        chars[k..].iter().position(|&c| c == ')')
                {
                    k += close + 1;
                }
                let statuses: BTreeSet<char> = chars[k..]
                    .iter()
                    .skip_while(|c| !matches!(**c, '⬜' | '🚧' | '✅'))
                    .take(1)
                    .copied()
                    .collect();
                out.insert(num, statuses);
            }
            i = j;
            continue;
        }
        i += 1;
    }
    out
}

/// Нормализует имя фичи: срезает префикс каталога (`features/`,
/// `docs/features/`), чтобы `features/testing.feature` и `testing.feature`
/// сравнивались одинаково (D80).
fn normalize_feature(token: &str) -> &str {
    token
        .strip_prefix("docs/features/")
        .or_else(|| token.strip_prefix("features/"))
        .unwrap_or(token)
}

/// Имена `*.feature`, упомянутые в тексте как `` `name.feature` ``
/// (в нормализованном виде — без префикса каталога).
fn feature_tokens(text: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for (idx, _) in text.match_indices('`') {
        let rest = &text[idx + 1..];
        if let Some(end) = rest.find('`') {
            let token = &rest[..end];
            if token.ends_with(".feature") && !token.contains(['*', '<', '{']) {
                out.insert(normalize_feature(token).to_string());
            }
        }
    }
    out
}

// --- реестры ----------------------------------------------------------------

/// Номера задач и статусы из `tasks/README.md` (таблица «Сводка»).
fn task_registry(text: &str) -> BTreeMap<u32, char> {
    let mut out = BTreeMap::new();
    for line in text.lines() {
        let line = line.trim();
        if !line.starts_with("| [T-") {
            continue;
        }
        let cells = table_cells(line);
        let id =
            find_refs(cells[0], 'T')
                .into_iter()
                .next()
                .unwrap_or_else(|| {
                    panic!("tasks/README: не разобран ID в строке: {line}")
                });
        let status = cells
            .last()
            .and_then(|c| c.chars().find(|c| matches!(*c, '⬜' | '🚧' | '✅')))
            .unwrap_or_else(|| {
                panic!("tasks/README: нет статуса в строке: {line}")
            });
        assert!(
            out.insert(id, status).is_none(),
            "tasks/README: дубликат задачи T-{id:02}"
        );
    }
    assert!(!out.is_empty(), "tasks/README: не найдено задач");
    out
}

/// Имена `*.feature` из таблиц `features/README.md` (в нормализованном виде —
/// без префикса каталога, см. `normalize_feature`).
fn feature_registry(text: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for (idx, _) in text.match_indices('`') {
        let rest = &text[idx + 1..];
        if let Some(end) = rest.find('`') {
            let token = &rest[..end];
            if token.ends_with(".feature") && !token.contains(['*', '<', '{']) {
                out.insert(normalize_feature(token).to_string());
            }
        }
    }
    assert!(!out.is_empty(), "features/README: не найдено ни одной фичи");
    out
}

/// Есть ли карточка задачи `docs/tasks/T-XX-*/README.md`.
fn task_card_exists(id: u32) -> bool {
    let dir = docs_dir().join("tasks");
    let prefix = format!("T-{id:02}-");
    fs::read_dir(&dir)
        .map(|entries| {
            entries.filter_map(|e| e.ok()).any(|e| {
                let name = e.file_name().to_string_lossy().into_owned();
                name.starts_with(&prefix)
                    && e.path().join("README.md").is_file()
            })
        })
        .unwrap_or(false)
}

// ---------------------------------------------------------------------------
// (1) Уникальность и целостность ID
// ---------------------------------------------------------------------------

#[test]
fn ids_are_unique_and_contiguous() {
    let q = numbered_files(&docs_dir().join("questions"), 'Q');
    let d = numbered_files(&docs_dir().join("decisions"), 'D');
    assert_contiguous(&q.keys().copied().collect(), "вопросы Q");
    assert_contiguous(&d.keys().copied().collect(), "решения D");
    assert!(
        q.len() >= 80 && d.len() >= 80,
        "ожидался полный журнал (Q{} / D{}), найдено Q{} / D{}",
        q.keys().next_back().unwrap(),
        d.keys().next_back().unwrap(),
        q.len(),
        d.len()
    );
}

// ---------------------------------------------------------------------------
// (2) Парность Q↔D
// ---------------------------------------------------------------------------

#[test]
fn questions_have_decision_or_exception() {
    let dir = docs_dir().join("questions");
    let q = numbered_files(&dir, 'Q');
    let d = numbered_files(&docs_dir().join("decisions"), 'D');

    for (num, path) in &q {
        let text = read(path);
        let status = field_line(&text, "Статус")
            .unwrap_or_else(|| panic!("Q{num}: нет поля «Статус»"));
        let resolved = status.starts_with("resolved by");
        let dropped = status.starts_with("dropped");
        let open = status.starts_with("open");
        let collateral = text.contains("закрыт попутно");
        assert!(
            resolved || dropped || open || collateral,
            "Q{num}: статус «{status}» не подкреплён решением/исключением"
        );
        if resolved {
            let refs = find_refs(status, 'D');
            assert_eq!(
                refs.len(),
                1,
                "Q{num}: `resolved by` без одного D: {status}"
            );
            let dn = *refs.iter().next().unwrap();
            assert!(
                d.contains_key(&dn),
                "Q{num}: ссылка на несуществующее D{dn}"
            );
        }
    }
}

#[test]
fn decisions_have_existing_question_or_retro_exception() {
    // Ретро-D без Q (D69): D1–D5, D7–D11, D13.
    let retro: BTreeSet<u32> =
        [1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 13].into_iter().collect();
    let q = numbered_files(&docs_dir().join("questions"), 'Q');

    for (num, path) in &numbered_files(&docs_dir().join("decisions"), 'D') {
        let text = read(path);
        let resolves = field_line(&text, "Resolves")
            .unwrap_or_else(|| panic!("D{num}: нет поля «Resolves»"));
        let refs = find_refs(resolves, 'Q');
        if refs.is_empty() {
            assert!(
                retro.contains(num),
                "D{num}: `Resolves: —` вне ретро-набора D69 (D1–D5, D7–D11, D13)"
            );
        } else {
            for qn in refs {
                assert!(
                    q.contains_key(&qn),
                    "D{num}: Resolves ссылается на несуществующий Q{qn}"
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// (3) Строки-представления в каталогах и TRACEABILITY
// ---------------------------------------------------------------------------

#[test]
fn decisions_are_listed_in_catalog() {
    let d = numbered_files(&docs_dir().join("decisions"), 'D');
    let catalog = read(&docs_dir().join("decisions").join("README.md"));
    let listed: BTreeSet<u32> = catalog
        .lines()
        .filter(|l| l.trim_start().starts_with("| [D"))
        .flat_map(|l| find_refs(l, 'D'))
        .collect();
    for num in d.keys() {
        assert!(
            listed.contains(num),
            "D{num}: нет строки в docs/decisions/README.md"
        );
    }
    for num in &listed {
        assert!(d.contains_key(num), "decisions/README: D{num} без файла");
    }
}

#[test]
fn questions_are_listed_in_catalog_and_traceability() {
    let q = numbered_files(&docs_dir().join("questions"), 'Q');
    let catalog = read(&docs_dir().join("questions").join("README.md"));
    let listed: BTreeSet<u32> = catalog
        .lines()
        .filter(|l| l.trim_start().starts_with("| [Q"))
        .flat_map(|l| find_refs(l, 'Q'))
        .collect();
    let trace = read(&docs_dir().join("TRACEABILITY.md"));
    let traced: BTreeSet<u32> = traceability_rows(&trace)
        .iter()
        .filter_map(|r| r.q)
        .collect();

    for num in q.keys() {
        assert!(
            listed.contains(num),
            "Q{num}: нет строки в docs/questions/README.md"
        );
        assert!(
            traced.contains(num),
            "Q{num}: нет строки в docs/TRACEABILITY.md"
        );
    }
    for num in &listed {
        assert!(q.contains_key(num), "questions/README: Q{num} без файла");
    }
    for num in &traced {
        assert!(
            q.contains_key(num),
            "TRACEABILITY: Q{num} без файла в docs/questions/"
        );
    }
}

// ---------------------------------------------------------------------------
// (4) Вердикт «Сверка с кодом» в каждом D
// ---------------------------------------------------------------------------

#[test]
fn every_decision_has_code_review_verdict() {
    for (num, path) in &numbered_files(&docs_dir().join("decisions"), 'D') {
        let text = read(path);
        let section = section(&text, "Сверка с кодом").unwrap_or_else(|| {
            panic!("D{num}: нет секции «## Сверка с кодом»")
        });
        assert!(
            section
                .lines()
                .any(|l| l.trim_start().starts_with("Вердикт:")),
            "D{num}: в «Сверке с кодом» нет строки «Вердикт:»"
        );
    }
}

// ---------------------------------------------------------------------------
// (5) Жизненный цикл TRACEABILITY
// ---------------------------------------------------------------------------

#[test]
fn traceability_lifecycle_uses_canon_dictionary() {
    let trace = read(&docs_dir().join("TRACEABILITY.md"));
    for (i, row) in traceability_rows(&trace).iter().enumerate() {
        let q = row.q.map(|n| format!("Q{n}")).unwrap_or_else(|| "?".into());
        assert!(
            matches!(row.lifecycle.as_str(), "open" | "in work" | "done"),
            "TRACEABILITY[{i}] {q}: недопустимый жизненный цикл «{}» (словарь D82: open/in work/done)",
            row.lifecycle
        );
        assert_ne!(
            row.lifecycle, "resolved",
            "TRACEABILITY[{i}] {q}: `resolved` упразднён (D82)"
        );
    }
}

/// Колонка `D` каждой строки `TRACEABILITY` ссылается на существующее решение;
/// единственное допустимое исключение — `open`-вопрос без принятого решения
/// (D63: строка = пара Q→D; D64 п.1.5).
#[test]
fn traceability_rows_reference_existing_decisions() {
    let d = numbered_files(&docs_dir().join("decisions"), 'D');
    let trace = read(&docs_dir().join("TRACEABILITY.md"));
    for (i, row) in traceability_rows(&trace).iter().enumerate() {
        let q = row.q.map(|n| format!("Q{n}")).unwrap_or_else(|| "?".into());
        match row.d {
            Some(dn) => assert!(
                d.contains_key(&dn),
                "TRACEABILITY[{i}] {q}: колонка D ссылается на несуществующее D{dn}"
            ),
            None => assert_eq!(
                row.lifecycle, "open",
                "TRACEABILITY[{i}] {q}: пустая колонка D при жизненном цикле «{}»",
                row.lifecycle
            ),
        }
    }
}

#[test]
fn traceability_lifecycle_matches_task_openness() {
    let trace = read(&docs_dir().join("TRACEABILITY.md"));
    for row in traceability_rows(&trace) {
        let tasks = cell_task_statuses(&row.tasks);
        let has_open = tasks
            .values()
            .any(|s| s.iter().copied().any(is_open_task_status));
        let q = row.q.map(|n| format!("Q{n}")).unwrap_or_else(|| "?".into());
        match row.lifecycle.as_str() {
            "in work" => assert!(
                has_open,
                "TRACEABILITY {q}: `in work` требует открытую задачу (⬜/🚧), а её нет"
            ),
            "done" | "open" => assert!(
                !has_open,
                "TRACEABILITY {q}: `{}` не допускает открытых задач",
                row.lifecycle
            ),
            _ => {},
        }
    }
}

// ---------------------------------------------------------------------------
// (6) Полнота задач (D77)
// ---------------------------------------------------------------------------

#[test]
fn traceability_tasks_exist_and_match_registry() {
    let registry =
        task_registry(&read(&docs_dir().join("tasks").join("README.md")));
    let trace = read(&docs_dir().join("TRACEABILITY.md"));
    let rows = traceability_rows(&trace);

    let mut traced: BTreeSet<u32> = BTreeSet::new();
    for row in &rows {
        for (id, statuses) in cell_task_statuses(&row.tasks) {
            traced.insert(id);
            assert!(
                task_card_exists(id),
                "TRACEABILITY: задача T-{id:02} без карточки в docs/tasks/"
            );
            let expected = *registry.get(&id).unwrap_or_else(|| {
                panic!("TRACEABILITY: T-{id:02} нет в реестре tasks/README.md")
            });
            for got in statuses {
                assert_eq!(
                    got, expected,
                    "TRACEABILITY: статус T-{id:02} «{got}» ≠ реестр «{expected}»"
                );
            }
        }
    }

    for (id, status) in &registry {
        assert!(
            traced.contains(id),
            "tasks/README: T-{id:02} ({status}) не видна ни в одной строке TRACEABILITY (D77)"
        );
    }
}

// ---------------------------------------------------------------------------
// (7) Полнота фич (D80)
// ---------------------------------------------------------------------------

#[test]
fn features_are_named_in_traceability_and_exist() {
    let registry =
        feature_registry(&read(&docs_dir().join("features").join("README.md")));
    let trace = read(&docs_dir().join("TRACEABILITY.md"));

    let mut named: BTreeSet<String> = BTreeSet::new();
    for row in traceability_rows(&trace) {
        for token in feature_tokens(&row.realization) {
            named.insert(token);
        }
        let realization = &row.realization;
        for wildcard in ["agents-*", "agents-*.feature", "и др."] {
            assert!(
                !realization.contains(wildcard),
                "TRACEABILITY: wildcard-обобщение «{wildcard}» недопустимо (D80): {realization}"
            );
        }
    }

    for name in &registry {
        assert!(
            named.contains(name),
            "features/README: {name} не упомянута поимённо в TRACEABILITY (D80)"
        );
        assert!(
            docs_dir().join("features").join(name).is_file(),
            "features/README: {name} объявлена, но файла нет"
        );
    }
    for name in &named {
        assert!(
            registry.contains(name),
            "TRACEABILITY: {name} не объявлена в features/README.md"
        );
        assert!(
            docs_dir().join("features").join(name).is_file(),
            "TRACEABILITY: {name} не существует в docs/features/"
        );
    }
}

// ---------------------------------------------------------------------------
// (8) Запреты D65
// ---------------------------------------------------------------------------

/// Канон журнала для проверок запретов (живые зоны; без исторических снимков).
fn journal_files() -> Vec<PathBuf> {
    let mut files = Vec::new();
    let questions = docs_dir().join("questions");
    let decisions = docs_dir().join("decisions");
    for dir in [&questions, &decisions] {
        for entry in fs::read_dir(dir)
            .unwrap_or_else(|e| panic!("нет каталога {}: {e}", dir.display()))
        {
            let path = entry.expect("запись каталога").path();
            if path.extension().and_then(|e| e.to_str()) == Some("md") {
                files.push(path);
            }
        }
    }
    for rel in [
        "TRACEABILITY.md",
        "tasks/README.md",
        "features/README.md",
        "SPECIFICATION.md",
    ] {
        files.push(docs_dir().join(rel));
    }
    files
}

/// Убирает секцию-снимок «## Сверка с кодом» из `D`-файла.
///
/// Carve-out — только для гейта **номеров строк** (D64 п.1.7 + D85 п.1):
/// «Сверка с кодом» — датированное свидетельство, приравненное к историческим
/// зонам карточки T-18 §Примечания. Адресный гейт (`removable_addresses`)
/// работает по сырому тексту: D64 п.1.8 / D65 п.4 строги и внутри «Сверки»
/// (D85 п.2).
fn live_text(text: &str) -> String {
    let start = match text.find("\n## Сверка с кодом") {
        Some(p) => p + 1,
        None => return text.to_string(),
    };
    let after = &text[start..];
    let end = after.find("\n## ").map(|p| p + 1).unwrap_or(after.len());
    let mut out = String::with_capacity(text.len());
    out.push_str(&text[..start]);
    out.push_str(&after[end..]);
    out
}

/// Адрес с номером строки: токен-путь + `:N` (опц. диапазон).
fn line_number_addresses(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let bytes: Vec<char> = text.chars().collect();
    for (i, &c) in bytes.iter().enumerate() {
        if c != ':' || i + 1 >= bytes.len() || !bytes[i + 1].is_ascii_digit() {
            continue;
        }
        // Назад — путь: буквы/цифры/`_./-`.
        let mut start = i;
        while start > 0
            && matches!(bytes[start - 1], 'A'..='Z' | 'a'..='z' | '0'..='9' | '_' | '.' | '/' | '-')
        {
            start -= 1;
        }
        let path: String = bytes[start..i].iter().collect();
        let looks_like_path = path.contains('/')
            || [".md", ".rs", ".feature", ".yaml", ".json", ".toml", ".dar"]
                .iter()
                .any(|ext| path.ends_with(ext));
        if looks_like_path {
            let mut end = i + 1;
            while end < bytes.len()
                && (bytes[end].is_ascii_digit()
                    || bytes[end] == '-'
                    || bytes[end] == '–'
                    || bytes[end] == ',')
            {
                end += 1;
            }
            let snippet: String = bytes[start..end].iter().collect();
            out.push(snippet);
        }
    }
    out
}

#[test]
fn no_line_number_addresses_in_live_journal() {
    let mut offenders = Vec::new();
    for path in journal_files() {
        let text = read(&path);
        let live = live_text(&text);
        for snippet in line_number_addresses(&live) {
            offenders.push(format!("{}: `{snippet}`", path.display()));
        }
    }
    assert!(
        offenders.is_empty(),
        "D65: адреса с номерами строк в живом журнале:\n  {}",
        offenders.join("\n  ")
    );
}

/// Шапка документа — до первого `## ` (для README/TRACEABILITY — весь текст).
fn header_block(text: &str) -> &str {
    match text.find("\n## ") {
        Some(p) => &text[..p],
        None => text,
    }
}

#[test]
fn no_migration_markers_in_live_journal() {
    let mut offenders = Vec::new();
    for path in journal_files() {
        let text = read(&path);
        for line in header_block(&text).lines() {
            for marker in ["ожидает переноса", "до конца миграции"]
            {
                if line.contains(marker) {
                    offenders.push(format!(
                        "{}: «{marker}» ({line})",
                        path.display()
                    ));
                }
            }
        }
        // `OPEN_QUESTIONS.md` запрещён как markdown-ссылка на удалённый файл
        // (упоминание текстом/в `Перенос:` — историч. метка, не ссылка).
        for line in text.lines() {
            if line.contains("OPEN_QUESTIONS.md)") && line.contains("](") {
                offenders.push(format!(
                    "{}: ссылка на удалённый OPEN_QUESTIONS.md",
                    path.display()
                ));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "D65/D61: миграционные маркеры в живом журнале:\n  {}",
        offenders.join("\n  ")
    );
}

/// Адрес конкретного файла в удаляемой/сессионной зоне (не упоминание папки).
/// Whitelist: `findings-registry.md` (D48). Возвращает `(сниппет, зона)`.
fn removable_addresses(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        for zone in [
            ".opencode/mail/",
            ".opencode/state/",
            "docs/research/",
            "docs/reviews/",
            "docs/analysis/",
        ] {
            let mut from = 0;
            while let Some(rel) = line[from..].find(zone) {
                let idx = from + rel + zone.len();
                // Имя файла: до пробела/скобки/обратной кавычки/конца строки.
                let name: String = line[idx..]
                    .chars()
                    .take_while(|c| {
                        !c.is_whitespace()
                            && !matches!(*c, ')' | '(' | '`' | ']' | ',' | '"')
                    })
                    .collect();
                let placeholder = name.contains("**")
                    || name.contains('*')
                    || name.contains("XX")
                    || name.contains('<')
                    || name.contains('{');
                let whitelisted = name == "findings-registry.md";
                let is_file = !name.ends_with('/') && name.contains('.');
                if is_file && !placeholder && !whitelisted {
                    out.push(format!("{zone}{name}"));
                }
                from = idx + name.len().max(1);
            }
        }
        // Относительные адреса вида `../analysis/<файл>` / `../../analysis/<файл>`.
        for zone in ["/analysis/", "/research/", "/reviews/"] {
            let mut from = 0;
            while let Some(rel) = line[from..].find(zone) {
                let idx = from + rel + zone.len();
                let name: String = line[idx..]
                    .chars()
                    .take_while(|c| {
                        !c.is_whitespace()
                            && !matches!(*c, ')' | '(' | '`' | ']' | ',' | '"')
                    })
                    .collect();
                let whitelisted = name == "findings-registry.md";
                let placeholder = name.contains("**") || name.contains('<');
                let dotted = line[..from + rel].ends_with("..");
                let is_file = !name.ends_with('/') && name.contains('.');
                if dotted && is_file && !placeholder && !whitelisted {
                    out.push(format!("..{zone}{name}"));
                }
                from = idx + name.len().max(1);
            }
        }
    }
    out
}

#[test]
fn no_addresses_to_removable_or_session_data() {
    let mut offenders = Vec::new();
    for path in journal_files() {
        let text = read(&path);
        for snippet in removable_addresses(&text) {
            offenders.push(format!("{}: {snippet}", path.display()));
        }
    }
    assert!(
        offenders.is_empty(),
        "D65: ссылки канона на удаляемые/сессионные данные:\n  {}",
        offenders.join("\n  ")
    );
}
