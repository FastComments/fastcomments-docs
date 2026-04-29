---
Срабатывает, когда модератор помечает комментарий как reviewed.

### Контекст, который получает агент

- Комментарий.
- **triggering user ID** - модератор, который просмотрел.
- Необязательный контекст: тред / история пользователя / контекст страницы в соответствии с настройками.

### Кто вызывает этот триггер

Действие человека-модератора на странице модерации, в виджете комментариев или через API.

### Распространённые сценарии использования

- **Пересылка аудита** через [Webhooks](#webhooks-overview).
- **Записи в память** — сохранение заметки в памяти о том, что этот комментарий был обработан человеком, чтобы другие агенты не обрабатывали его повторно.

### Примечательно

- "Reviewed" is one of the moderation queue states tracked separately from "approved" and "spam". A comment can be approved-and-reviewed, approved-but-not-reviewed, etc. See [How Approvals Work](/guide-moderation.html#moderation-approvals) in the moderation guide.
- This trigger is high-frequency on tenants with many moderators. Subscribe selectively and budget accordingly.

---