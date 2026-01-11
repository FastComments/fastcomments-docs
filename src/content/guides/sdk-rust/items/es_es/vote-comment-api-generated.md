## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| comment_id | String | Sí |  |
| url_id | String | Sí |  |
| broadcast_id | String | Sí |  |
| vote_body_params | models::VoteBodyParams | Sí |  |
| session_id | String | No |  |
| sso | String | No |  |

## Respuesta

Devuelve: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_comment_200_response.rs)

---