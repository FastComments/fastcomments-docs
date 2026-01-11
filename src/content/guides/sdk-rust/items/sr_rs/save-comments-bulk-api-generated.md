## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| create_comment_params | Vec<models::CreateCommentParams> | Да |  |
| is_live | bool | Не |  |
| do_spam_check | bool | Не |  |
| send_emails | bool | Не |  |
| populate_notifications | bool | Не |  |

## Одговор

Враћа: `Vec<models::SaveComment200Response>`

---