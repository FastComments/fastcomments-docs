## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| comment_id | String | Sí |  |
| broadcast_id | String | Sí |  |
| comment_text_update_request | models::CommentTextUpdateRequest | Sí |  |
| edit_key | String | No |  |
| sso | String | No |  |

## Respuesta

Devuelve: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_text_200_response.rs)

---