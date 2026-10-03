# ideas/ — идеи интерфейса DAR Notebook

Сырые макеты и концепты UI: **идеи на подумать**, не окончательные решения и не
требования. При разработке интерфейса сюда заглядывают за формой панелей,
потоками и деталями; лучшее переносится в требования обычным циклом.

## Статус и правила жизни

- **Не канон** ([D60](../decisions/D60-docs-ownership-sync.md)): канон на файлы
  этой папки не ссылается, факты живут в Q/D, задачах, фичах (по духу
  [D65](../decisions/D65-reference-policy.md)).
- **Ссылки — только «отсюда → канон»:** этот README ведёт на требования и
  спеку; обратные ссылки из канона не заводятся.
- **Не требования:** за реализацию отвечают
  [`features/`](../features/README.md); идея ≠ обещание.
- **Долгоживущий референс:** в отличие от рабочих артефактов `analysis/`,
  `reviews/`, `research/`, файлы здесь штатно не удаляются.
- **Внутренние макеты**, а не внешние обзоры — поэтому не в `research/`.

## Макеты

| Файл | Направление | Что показывает | Связь с требованиями |
|---|---|---|---|
| [`ui/dar-notebook-v0.html`](ui/dar-notebook-v0.html) | CREDO — DAR Notebook | Базовый трёхколоночный layout: «Файлы правил», «Версии» (git-теги active/supported/deprecated), редактор с подсветкой, «Результат исполнения», «AI Ассистент», модалка «Тестовые данные» | [`notebook_ui`](../features/notebook_ui.feature), [`editor`](../features/editor.feature), [`inline_execution`](../features/inline_execution.feature), [`agent_minimal`](../features/agent_minimal.feature), [`git_integration`](../features/git_integration.feature) |
| [`ui/dar-notebook-v1.html`](ui/dar-notebook-v1.html) | CREDO — DAR Notebook (расширенная) | То же направление детальнее: i18n RU/EN, редактор на CodeMirror с языковой поддержкой, модалка «Публикация правила», тосты (создано/опубликовано/удалено, тесты парсера), история версий | [`notebook_ui`](../features/notebook_ui.feature), [`editor`](../features/editor.feature), [`inline_execution`](../features/inline_execution.feature), [`agent_minimal`](../features/agent_minimal.feature), [`git_integration`](../features/git_integration.feature) |
| [`ui/riskforge-v0.html`](ui/riskforge-v0.html) | RiskForge — Banking Rules Engine | Альтернативное направление (англ. UI, свой CSS): хедер Rules/Settings, левая панель «Business Rules» и «Git History», центральный редактор, правая панель «Test Execution» (табы Results/History, «Test Scenarios», Run Tests), кнопки Preview/Save Explanation, Run/Deploy | Альтернативный взгляд на [`notebook_ui`](../features/notebook_ui.feature), [`inline_execution`](../features/inline_execution.feature), [`git_integration`](../features/git_integration.feature) |

> `dar-notebook-v1.html` демонстрирует и заведомо вне-MVP конструкции (`И`,
> `ИЛИ`, `Иначе`, `ИначеЕсли`, `НЕ`) — как идею, куда может вырасти язык; в
> v0.1 их нет ([`GRAMMAR.md`](../GRAMMAR.md)).
>
> Выбор направления (Notebook vs RiskForge) — за владельцем; макеты
> равноправны, это не решение.

## Как пользоваться

- Открыть `.html` в браузере. Нужен интернет: шрифты из CDN; `dar-notebook-v0`
  дополнительно тянет Tailwind, `dar-notebook-v1` — CodeMirror с `esm.sh`,
  `riskforge-v0` — только шрифты.
- Чтобы идея стала каноном, её формулируют как требование в
  [`features/*.feature`](../features/README.md) обычным циклом Q/D; сам макет
  при этом остаётся референсом.

## Происхождение

Перенесено 2026-10-03 из `prototypes/front_ideas/`:

| Было | Стало |
|---|---|
| `interface_type1_0.html` | `ui/dar-notebook-v0.html` |
| `interface_type1_1.html` | `ui/dar-notebook-v1.html` |
| `interface_type2_0.html` | `ui/riskforge-v0.html` |

При переносе совпадение содержимого сверено по SHA-256; исходная папка удалена
(архив — git-история).
