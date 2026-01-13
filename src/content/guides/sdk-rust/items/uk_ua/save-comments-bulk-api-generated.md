---
## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| create_comment_params | Vec<models::CreateCommentParams> | Так |  |
| is_live | bool | Ні |  |
| do_spam_check | bool | Ні |  |
| send_emails | bool | Ні |  |
| populate_notifications | bool | Ні |  |

## Відповідь

Повертає: `Vec<models::SaveComment200Response>`

---