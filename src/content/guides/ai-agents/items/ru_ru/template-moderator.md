**Template ID:** `tos_enforcer`

Шаблон модератора — рекомендуемая отправная точка, если ваша цель — уменьшить ручную модерацию. Он проверяет новые и отмеченные флаги комментарии и применяет правила вашего сообщества.

### Встроенный начальный запрос

[inline-code-attrs-start title = 'Начальный запрос шаблона модератора'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a terms-of-service enforcement agent. Review each new comment against the community guidelines. Mark clear spam or policy violations. Issue warnings for first-offense borderline content. Escalate ban decisions only for repeat or severe violations. If a comment is clearly within the rules, approve it so it becomes visible (relevant for pre-moderation tenants). Stay out of political or subjective debates, focus on the rules as written.
[inline-code-end]

Вы почти всегда захотите **дополнить этот запрос** конкретными примерами того, что на вашем сайте разрешено и что нет. Собственная политика эскалации платформы (предупреждать перед баном, искать в памяти перед блокировкой) уже учтена в системном запросе, который получает агент, поэтому повторять её не нужно.

### Триггеры

- **New comment posted** (`COMMENT_ADD`) - агент проверяет каждый новый комментарий.
- **Comment crosses a flag threshold** (`COMMENT_FLAG_THRESHOLD`, default threshold: 3) - агент повторно оценивает комментарий, который другие пользователи пометили.

### Разрешённые инструменты

- [`mark_comment_approved`](#tools-overview) - полезно для тенантов с предмодерацией, где агент публикует чистые комментарии и скрывает остальные.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Он не может публиковать комментарии, голосовать, прикреплять, блокировать, присваивать значки или отправлять электронную почту — запрос намеренно ограничен.

### Рекомендуемые дополнения перед запуском

- **Set [Community Guidelines](#community-guidelines).** Нескольких предложений письменной политики достаточно; агент применяет её при каждом запуске.
- **Gate `ban_user` behind [approval](#approval-workflow).** Эта опция включена по умолчанию в регионе ЕС (см. [EU DSA Article 17 Compliance](#eu-dsa-compliance)) и рекомендуется везде.
- **Consider also gating `mark_comment_spam` behind approval** если у вас низкий объём, но высокие ставки контента.
- **Gate `mark_comment_approved` behind approval if you run pre-moderation.** Одобрение плохого комментария ставит его перед читателями; ограничьте эту возможность, пока агент не заслужит доверие в режиме "сухого запуска".
- **Tick "Include commenter's trust factor, account age, ban history, and recent comments"** в [Context Options](#context-options). Модель будет реже давать излишние предупреждения, когда сможет видеть, что пользователь — давний добросовестный участник.

### Рекомендуемое окно для dry-run

Запускайте этот шаблон в режиме [dry-run](#dry-run-mode) как минимум в течение недели на реальном трафике перед включением. Используйте [Test Runs (Replays)](#test-runs-replays), чтобы также просмотреть результаты за предыдущие 30 дней.

---