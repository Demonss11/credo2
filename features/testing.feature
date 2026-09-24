# language: ru
Функция: Тестирование
  Как разработчик
  Я хочу покрыть критичные пути тестами
  Чтобы рефакторинг не ломал контракт

  Сценарий: Юнит-тесты semver покрывают граничные случаи
    Тогда существуют тесты:
      | имя                              |
      | parse_rejects_v_dot              |
      | parse_accepts_v_prefix           |
      | prerelease_sorts_below_release   |
      | compare_semver_returns_error     |

  Сценарий: Интеграционные тесты публикации используют временный git-репо
    Тогда существуют тесты:
      | имя                                    |
      | rejects_duplicate_version              |
      | rejects_downgrade                      |
      | rejects_contract_change_without_major  |
      | manifest_hash_is_deterministic         |
      | meta_version_matches_path              |

  Сценарий: CI блокирует merge при нарушениях
    Когда запускается CI на PR
    Тогда выполняется "cargo fmt --check"
    И выполняется "cargo clippy -- -D warnings"
    И выполняется "cargo test"
    И если любой шаг падает, merge блокируется
