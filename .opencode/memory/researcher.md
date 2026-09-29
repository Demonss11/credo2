# Память: researcher (внешние обзоры)

- **Канон:** `docs/research/`; глоссарий `SPECIFICATION.md` §11.
- **Правило:** чекпойнт — тема, файл обзора, источники, что осталось. Кратко.

## Чекпойнты

- **2026-09-29 · service-doc-tools · завершено.** Тема: doc-quality тулсет
  соседнего проекта. Файл: `docs/research/doc-quality-checks-2026-09-29.md`
  (draft, ~170 строк). Источники: дайджест сервисной сессии + markdownlint-cli2,
  cspell, markdown-link-check, lychee, Docsie/Fern/Netlify. Ключевой вывод:
  size+lint — брать/адаптировать; cspell — позже; link-check (npm) — отклонить,
  владелец — Rust T-18 v0.2; changelog-gen — отклонить. Не делал: пилот
  (нет shell/git по правам роли; `cargo` — D50), внешние ссылки не проверял.
- **Урок:** согласие «история — снимок» (T-18/D64) задаёт зоны-исключения для
  любого doc-линтера; проверка не должна дублировать T-18 — разделение
  «структура (Rust) ↔ форма (lint) ↔ размер (size)».
- **2026-09-29 · правка P3 (validator).** Обзор ссылался на
  `.opencode/mail/service-doc-tools.md` — риск F47 (адрес умрёт после
  `clean-logs --mail-only`, D65). Исправлено: имя без URL. **Правило себе:**
  в `docs/research/**` на рабочие данные (`mail/`, `state/`) ссылаться только
  именем в code-span, никогда markdown-ссылкой.

