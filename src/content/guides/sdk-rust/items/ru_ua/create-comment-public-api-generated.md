---
## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| url_id | String | Да |  |
| broadcast_id | String | Да |  |
| comment_data | models::CommentData | Да |  |
| session_id | String | Нет |  |
| sso | String | Нет |  |

## Ответ

Возвращает: [`CreateCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_comment_public_200_response.rs)

---