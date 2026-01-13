---
## Parametri

| Ime | Tip | Zahtevano | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| comment_id | String | Da |  |
| broadcast_id | String | Da |  |
| comment_text_update_request | models::CommentTextUpdateRequest | Da |  |
| edit_key | String | Ne |  |
| sso | String | Ne |  |

## Odgovor

Vrne: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_text_200_response.rs)

---