---
## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| comment_id | String | Sì |  |
| url_id | String | Sì |  |
| broadcast_id | String | Sì |  |
| vote_body_params | models::VoteBodyParams | Sì |  |
| session_id | String | No |  |
| sso | String | No |  |

## Risposta

Restituisce: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_comment_200_response.rs)

---