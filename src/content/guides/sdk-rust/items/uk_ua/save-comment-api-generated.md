## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| create_comment_params | models::CreateCommentParams | Так |  |
| is_live | bool | Ні |  |
| do_spam_check | bool | Ні |  |
| send_emails | bool | Ні |  |
| populate_notifications | bool | Ні |  |

## Відповідь

Повертає: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/save_comment_200_response.rs)

---