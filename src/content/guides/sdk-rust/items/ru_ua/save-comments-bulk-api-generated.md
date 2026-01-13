---
## Параметры

| Имя | Тип | Обязателен | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| create_comment_params | Vec<models::CreateCommentParams> | Да |  |
| is_live | bool | Нет |  |
| do_spam_check | bool | Нет |  |
| send_emails | bool | Нет |  |
| populate_notifications | bool | Нет |  |

## Ответ

Возвращает: `Vec<models::SaveComment200Response>`

---