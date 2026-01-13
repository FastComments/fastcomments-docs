## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| comment_id | String | Sì |  |
| broadcast_id | String | Sì |  |
| comment_text_update_request | models::CommentTextUpdateRequest | Sì |  |
| edit_key | String | No |  |
| sso | String | No |  |

## Risposta

Restituisce: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_text_200_response.rs)

---