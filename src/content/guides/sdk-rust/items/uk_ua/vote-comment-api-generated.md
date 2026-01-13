---
## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| comment_id | String | Так |  |
| url_id | String | Так |  |
| broadcast_id | String | Так |  |
| vote_body_params | models::VoteBodyParams | Так |  |
| session_id | String | Ні |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_comment_200_response.rs)

---