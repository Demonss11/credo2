//! Q40: инвентаризация Gherkin-фич (`docs/features/*.feature`).
//!
//! Фичи — документация, а не исполняемая спецификация, поэтому тест не
//! исполняет сценарии, а проверяет целостность документации: структуру
//! файлов, полноту перечня и счётчики, совпадающие с `docs/features/README.md`.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Разобранная фича.
#[derive(Debug)]
struct FeatureFile {
    name: String,
    scenarios: usize,
}

/// Каталог фич: документация перенесена в `docs/features` (2026-09-26).
fn features_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("docs")
        .join("features")
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("не читается {}: {e}", path.display()))
}

/// Собирает все `*.feature` и проверяет их базовую структуру.
fn collect_features() -> Vec<FeatureFile> {
    let dir = features_dir();
    let mut files: Vec<FeatureFile> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("нет каталога {}: {e}", dir.display()))
        .map(|entry| entry.expect("не читается запись каталога features").path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("feature"))
        .map(|path| {
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            let text = read(&path);

            let first = text.lines().next().unwrap_or_default().trim();
            assert_eq!(
                first, "# language: ru",
                "{name}: первая строка должна быть '# language: ru'"
            );

            let functions = text
                .lines()
                .filter(|line| line.trim_start().starts_with("Функция:"))
                .count();
            assert_eq!(functions, 1, "{name}: должна быть ровно одна 'Функция:'");

            let scenarios = text
                .lines()
                .filter(|line| {
                    let line = line.trim_start();
                    line.starts_with("Сценарий:") || line.starts_with("Структура сценария:")
                })
                .count();
            assert!(scenarios > 0, "{name}: нет ни одного сценария");

            FeatureFile { name, scenarios }
        })
        .collect();

    files.sort_by(|a, b| a.name.cmp(&b.name));
    assert!(
        !files.is_empty(),
        "в docs/features/ нет ни одного .feature файла"
    );
    files
}

/// Счётчики сценариев из сводных таблиц README (имя файла → сценариев).
fn readme_counts(text: &str) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for line in text.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix("| [`") else {
            continue;
        };
        // Строка вида:
        // | [`name.feature`](name.feature) | Категория | N | ⬜ | 🔴 | ... |
        let Some((name, _)) = rest.split_once('`') else {
            continue;
        };
        if !name.ends_with(".feature") {
            continue;
        }

        let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
        assert!(cells.len() >= 3, "{name}: строка README короче таблицы");
        let scenarios: usize = cells[2]
            .parse()
            .unwrap_or_else(|_| panic!("{name}: в README не число сценариев: {:?}", cells[2]));
        assert!(
            counts.insert(name.to_string(), scenarios).is_none(),
            "{name}: в README несколько строк для одного файла"
        );
    }
    assert!(
        !counts.is_empty(),
        "в docs/features/README.md не найдено ни одной строки таблицы"
    );
    counts
}

fn readme_text() -> String {
    read(&features_dir().join("README.md"))
}

#[test]
fn feature_files_are_valid_documents() {
    let files = collect_features();
    assert!(
        files.len() >= 30,
        "ожидалось не меньше 30 фич, найдено {}",
        files.len()
    );
}

#[test]
fn feature_files_match_readme_inventory() {
    let files = collect_features();
    let counts = readme_counts(&readme_text());

    for file in &files {
        assert!(
            counts.contains_key(&file.name),
            "{}: файл есть, но не указан в docs/features/README.md",
            file.name
        );
    }
    for name in counts.keys() {
        assert!(
            files.iter().any(|file| file.name == *name),
            "{name}: указан в README, но файла нет"
        );
    }
}

#[test]
fn scenario_counts_match_readme() {
    let files = collect_features();
    let counts = readme_counts(&readme_text());

    for file in &files {
        assert_eq!(
            counts.get(&file.name).copied(),
            Some(file.scenarios),
            "{}: счётчик сценариев расходится с README",
            file.name
        );
    }
}

#[test]
fn readme_totals_match_files() {
    let files = collect_features();
    let total_scenarios: usize = files.iter().map(|file| file.scenarios).sum();
    let text = readme_text();

    let line = text
        .lines()
        .find(|line| line.contains("Итого:"))
        .expect("в README нет строки «Итого:»");
    let numbers: Vec<usize> = line
        .split(|c: char| !c.is_ascii_digit())
        .filter(|part| !part.is_empty())
        .map(|part| part.parse().expect("число в строке «Итого»"))
        .collect();

    assert!(numbers.len() >= 2, "в строке «Итого» нет чисел: {line}");
    assert_eq!(numbers[0], files.len(), "README: неверное число файлов");
    assert_eq!(
        numbers[1], total_scenarios,
        "README: неверное число сценариев"
    );
}
