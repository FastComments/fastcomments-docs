---
## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| create_comment_params | models::CreateCommentParams | Да |  |
| is_live | bool | Не |  |
| do_spam_check | bool | Не |  |
| send_emails | bool | Не |  |
| populate_notifications | bool | Не |  |

## Отговор

Връща: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/save_comment_200_response.rs)

---