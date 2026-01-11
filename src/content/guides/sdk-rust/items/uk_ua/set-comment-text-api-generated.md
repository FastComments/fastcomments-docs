## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| comment_id | String | Так |  |
| broadcast_id | String | Так |  |
| comment_text_update_request | models::CommentTextUpdateRequest | Так |  |
| edit_key | String | Ні |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_text_200_response.rs)