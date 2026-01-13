## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| comment_id | String | Да |  |
| broadcast_id | String | Да |  |
| comment_text_update_request | models::CommentTextUpdateRequest | Да |  |
| edit_key | String | Нет |  |
| sso | String | Нет |  |

## Ответ

Возвращает: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_text_200_response.rs)

---