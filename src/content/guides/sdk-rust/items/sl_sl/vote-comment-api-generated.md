## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| comment_id | String | Da |  |
| url_id | String | Da |  |
| broadcast_id | String | Da |  |
| vote_body_params | models::VoteBodyParams | Da |  |
| session_id | String | Ne |  |
| sso | String | Ne |  |

## Odgovor

Vrne: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_comment_200_response.rs)

---