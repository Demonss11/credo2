use credo2::core::{Rule, contract_from_rule, parse_rule};
use credo2::{
    commit_tree, ensure_repo, list_from_ref, publish, rev_parse, update_ref,
    write_index_with_parent,
};

fn tmp_repo() -> tempfile::TempDir {
    let t = tempfile::tempdir().unwrap();
    ensure_repo(t.path()).unwrap();
    t
}

fn rule_with(field: &str) -> Rule {
    parse_rule(&format!(
        "Правило CreditAgeMin {{ Если ({field} < 21) {{ Решение = Отказ; Причина = \"x\"; }} }}"
    ))
    .unwrap()
}

#[test]
fn rejects_duplicate_version() {
    let t = tmp_repo();
    let r = rule_with("Клиент.Возраст");
    let c = contract_from_rule(&r, "1.0.0");
    publish(t.path(), &r, &c, "1.0.0", "test").unwrap();
    // merge в main вручную (для теста)
    merge_branch_to_main(t.path(), "publish/CreditAgeMin-1.0.0");
    let err = publish(t.path(), &r, &c, "1.0.0", "test").unwrap_err();
    assert!(err.to_string().contains("уже существует"));
}

#[test]
fn rejects_downgrade() {
    let t = tmp_repo();
    let r = rule_with("Клиент.Возраст");
    let c = contract_from_rule(&r, "1.0.0");
    publish(t.path(), &r, &c, "1.0.0", "test").unwrap();
    merge_branch_to_main(t.path(), "publish/CreditAgeMin-1.0.0");
    let err = publish(t.path(), &r, &c, "0.9.0", "test").unwrap_err();
    assert!(err.to_string().contains("не больше максимальной"));
    assert!(err.to_string().contains("downgrade запрещён"));
}

#[test]
fn rejects_contract_change_without_major() {
    let t = tmp_repo();
    let r1 = rule_with("Клиент.Возраст");
    let c1 = contract_from_rule(&r1, "1.0.0");
    publish(t.path(), &r1, &c1, "1.0.0", "test").unwrap();
    merge_branch_to_main(t.path(), "publish/CreditAgeMin-1.0.0");

    let r2 = rule_with("Клиент.Доход");
    let c2 = contract_from_rule(&r2, "1.1.0");
    let err = publish(t.path(), &r2, &c2, "1.1.0", "test").unwrap_err();
    assert!(err.to_string().contains("контракт изменился"));
    assert!(err.to_string().contains("MAJOR"));
}

#[test]
fn allows_major_bump_with_contract_change() {
    let t = tmp_repo();
    let r1 = rule_with("Клиент.Возраст");
    publish(
        t.path(),
        &r1,
        &contract_from_rule(&r1, "1.0.0"),
        "1.0.0",
        "test",
    )
    .unwrap();
    merge_branch_to_main(t.path(), "publish/CreditAgeMin-1.0.0");

    let r2 = rule_with("Клиент.Доход");
    publish(
        t.path(),
        &r2,
        &contract_from_rule(&r2, "2.0.0"),
        "2.0.0",
        "test",
    )
    .unwrap();

    let list = list_from_ref(t.path(), "publish/CreditAgeMin-2.0.0").unwrap();
    assert!(list.iter().any(|c| c.version == "2.0.0"));
    assert!(list.iter().any(|c| c.version == "1.0.0"));
}

#[test]
fn allows_patch_without_contract_change() {
    let t = tmp_repo();
    let r = rule_with("Клиент.Возраст");
    publish(
        t.path(),
        &r,
        &contract_from_rule(&r, "1.0.0"),
        "1.0.0",
        "test",
    )
    .unwrap();
    merge_branch_to_main(t.path(), "publish/CreditAgeMin-1.0.0");
    publish(
        t.path(),
        &r,
        &contract_from_rule(&r, "1.0.1"),
        "1.0.1",
        "test",
    )
    .unwrap();
}

#[test]
fn meta_version_matches_path() {
    let t = tmp_repo();
    let r = rule_with("Клиент.Возраст");
    publish(
        t.path(),
        &r,
        &contract_from_rule(&r, "1.0.0"),
        "1.0.0",
        "test",
    )
    .unwrap();
    merge_branch_to_main(t.path(), "publish/CreditAgeMin-1.0.0");
    let checks = list_from_ref(t.path(), "main").unwrap();
    assert_eq!(checks.len(), 1);
    assert_eq!(checks[0].meta.version, "1.0.0");
    assert_eq!(checks[0].meta.name, "CreditAgeMin");
    assert!(checks[0].meta.checksum.starts_with("sha256:"));
}

#[test]
fn manifest_hash_is_deterministic() {
    let t = tmp_repo();
    let r = rule_with("Клиент.Возраст");
    publish(
        t.path(),
        &r,
        &contract_from_rule(&r, "1.0.0"),
        "1.0.0",
        "test",
    )
    .unwrap();
    merge_branch_to_main(t.path(), "publish/CreditAgeMin-1.0.0");

    let c = list_from_ref(t.path(), "main").unwrap();
    let m1 = credo2::build_manifest(&c);
    let m2 = credo2::build_manifest(&c);
    assert_eq!(m1.service_hash, m2.service_hash);
}

#[test]
fn publish_normalizes_contract_version() {
    let t = tmp_repo();
    let r = rule_with("Клиент.Возраст");
    let raw = contract_from_rule(&r, "v1.0.0"); // сырой, с префиксом
    publish(t.path(), &r, &raw, "v1.0.0", "test").unwrap();
    merge_branch_to_main(t.path(), "publish/CreditAgeMin-1.0.0");

    let checks = list_from_ref(t.path(), "main").unwrap();
    assert_eq!(checks.len(), 1);
    assert_eq!(checks[0].contract.version, "1.0.0");
    assert_eq!(checks[0].version, "1.0.0");
    assert_eq!(checks[0].meta.version, "1.0.0");
}

#[test]
fn build_metadata_excluded_from_path() {
    let t = tmp_repo();
    let r = rule_with("Клиент.Возраст");
    publish(
        t.path(),
        &r,
        &contract_from_rule(&r, "1.2.3+build.7"),
        "1.2.3+build.7",
        "test",
    )
    .unwrap();
    merge_branch_to_main(t.path(), "publish/CreditAgeMin-1.2.3");

    let checks = list_from_ref(t.path(), "main").unwrap();
    assert_eq!(checks.len(), 1);
    assert_eq!(checks[0].version, "1.2.3");
    assert_eq!(checks[0].contract.version, "1.2.3");
}

#[test]
fn merge_preserves_other_checks() {
    let t = tmp_repo();
    let r1 = rule_with("Клиент.Возраст");
    publish(
        t.path(),
        &r1,
        &contract_from_rule(&r1, "1.0.0"),
        "1.0.0",
        "test",
    )
    .unwrap();
    merge_branch_to_main(t.path(), "publish/CreditAgeMin-1.0.0");

    let r2 = parse_rule(
        "Правило OtherCheck { Если (Сумма < 100) { Решение = Отказ; Причина = \"x\"; } }",
    )
    .unwrap();
    publish(
        t.path(),
        &r2,
        &contract_from_rule(&r2, "1.0.0"),
        "1.0.0",
        "test",
    )
    .unwrap();
    merge_branch_to_main(t.path(), "publish/OtherCheck-1.0.0");

    let checks = list_from_ref(t.path(), "main").unwrap();
    assert_eq!(checks.len(), 2); // ловит squash-merge
}

#[test]
fn cyrillic_check_name_is_listed() {
    let t = tmp_repo();
    let src = "Правило МинимальныйВозраст { Если (Клиент.Возраст < 21) { Решение = Отказ; Причина = \"x\"; } }";
    let r = parse_rule(src).unwrap();
    publish(
        t.path(),
        &r,
        &contract_from_rule(&r, "1.0.0"),
        "1.0.0",
        "test",
    )
    .unwrap();
    merge_branch_to_main(t.path(), "publish/МинимальныйВозраст-1.0.0");

    // git ls-tree экранирует кириллицу, если не запросить `-z` (регрессия).
    let checks = list_from_ref(t.path(), "main").unwrap();
    assert_eq!(checks.len(), 1);
    assert_eq!(checks[0].name, "МинимальныйВозраст");
    assert_eq!(checks[0].version, "1.0.0");

    let m = credo2::build_manifest(&checks);
    assert_eq!(m.checks.len(), 1);
    assert_eq!(m.checks[0].active, "1.0.0");
}

#[test]
fn create_ref_prevents_double_publish() {
    let t = tmp_repo();
    let r = rule_with("Клиент.Возраст");
    let c = contract_from_rule(&r, "1.0.0");
    publish(t.path(), &r, &c, "1.0.0", "test").unwrap();
    // main не обновляем: ранняя проверка дубликата не срабатывает,
    // падать должен именно create_ref на уже существующей ветке.
    let err = publish(t.path(), &r, &c, "1.0.0", "test").unwrap_err();
    assert!(err.to_string().contains("ветка"), "err = {err}");
}

// merge_branch_to_main — тестовый хелпер, эмулирует merge PR.
fn merge_branch_to_main(repo: &std::path::Path, branch: &str) {
    let tree = write_index_with_parent(repo, branch, &[]).unwrap();
    let main = rev_parse(repo, "main").unwrap();
    let commit = commit_tree(repo, &tree, &main, "merge").unwrap();
    update_ref(repo, "refs/heads/main", &commit).unwrap();
}
