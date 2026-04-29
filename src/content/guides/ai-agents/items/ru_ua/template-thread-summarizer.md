**ID шаблона:** `thread_summarizer`

The Thread Summarizer posts a neutral, single-paragraph summary at the end of a long thread. It uses a 30-minute deferral so the thread can settle before the agent looks at it.

### Встроенный начальный запрос

[inline-code-attrs-start title = 'Исходный запрос шаблона Thread Summarizer'; type='text' inline-code-attrs-end]
[inline-code-start]
You post neutral thread summaries. Do not summarize threads that have fewer than 5 comments. For longer threads, summarize the main positions, disagreements, and open questions in one short paragraph. Do not take sides and do not editorialize. After posting the summary, pin it. If a prior summary by you is already pinned on this thread, unpin it before pinning the new one.
[inline-code-end]

Инструкция "do not editorialize" имеет ключевое значение. Без неё модель склонна к формулировкам типа "in my view", которые плохо смотрятся под именем вашего аккаунта.

### Триггеры

- **Новый комментарий добавлен** (`COMMENT_ADD`).
- **Задержка триггера**: 30 минут (1800 секунд). См. [Deferred Triggers](#trigger-deferred-delay).

Задержка в 30 минут означает, что агент запускается один раз, через полчаса после появления комментария, и работает с тем видом треда, который есть в этот момент. Это не означает "суммировать каждый ответ" — очередь отложенных триггеров объединяет несколько событий нового комментария в одном треде, но не декуплирует их по отдельным окнам. Вероятно, вы захотите **добавить в свой промпт собственное правило** вроде "не публиковать новую сводку, если агент уже суммировал этот тред за последние 24 часа" и опираться на контекст плюс [tools-overview](#tools-overview) агента для его соблюдения.

### Разрешённые инструменты

- [`write_comment`](#tools-overview) - публикует саму сводку.
- [`pin_comment`](#tools-overview) - закрепляет сводку, чтобы читатели видели её вверху треда.
- [`unpin_comment`](#tools-overview) - снимает закрепление предыдущей сводки тем же агентом перед закреплением новой.

Суммаризатор не может модерировать или взаимодействовать с пользователями.

### Закрепление сводки

Агент публикует новый комментарий с помощью `write_comment`, затем вызывает `pin_comment` с возвращённым ID комментария. При последующих запусках на том же треде промпт инструктирует его сначала вызвать `unpin_comment` для своей предыдущей сводки — сама платформа **не** обеспечивает правило "только один закреплённый комментарий на тред", поэтому оставление предыдущей сводки закреплённой приведёт к появлению двух закреплённых сводок рядом. Отметьте параметр "Include parent comment and prior replies in the same thread" в [Context Options](#context-options), чтобы агент видел предыдущую закреплённую сводку.

### Рекомендуемые дополнения перед запуском

- **Отметьте параметр "Include parent comment and prior replies in the same thread"** в [Context Options](#context-options). Суммаризатор без контекста треда бесполезен.
- **Настройте правило минимального размера треда.** "Fewer than 5 comments" — значение по умолчанию в промпте, но в активных сообществах более подходящим будет диапазон 10–20. Отредактируйте промпт напрямую.
- **Ограничьте по конкретным шаблонам URL**, если вы хотите сводки только на страницах длинного формата, а не на объявлениях или страницах продуктов. См. [Scope: URL and Locale Filters](#scope-url-locale).
- **Следите за расходами.** Сводки наиболее ресурсоёмки по токенам, так как агент читает весь тред при каждом запуске. Установите жёсткий [ежедневный бюджет](#budgets-overview) перед включением режима Enabled.

### Избежание повторных сводок

Агент имеет доступ к [`save_memory`](#tools-overview) и [`search_memory`](#tools-overview) — вы можете расширить промпт, заставив его записывать заметки вроде "summarized {thread urlId}" и проверять их перед повторной публикацией. Память общая для всех агентов вашего тенанта.

---