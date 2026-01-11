## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| comment_id | String | Tak |  |
| url_id | String | Tak |  |
| broadcast_id | String | Tak |  |
| vote_body_params | models::VoteBodyParams | Tak |  |
| session_id | String | Nie |  |
| sso | String | Nie |  |

## Odpowiedź

Zwraca: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_comment_200_response.rs)

---