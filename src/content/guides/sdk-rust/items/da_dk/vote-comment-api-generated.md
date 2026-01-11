## Parametre

| Navn | Type | Obligatorisk | Beskrivelse |
|------|------|--------------|-------------|
| tenant_id | String | Ja |  |
| comment_id | String | Ja |  |
| url_id | String | Ja |  |
| broadcast_id | String | Ja |  |
| vote_body_params | models::VoteBodyParams | Ja |  |
| session_id | String | Nej |  |
| sso | String | Nej |  |

## Svar

Returnerer: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_comment_200_response.rs)

---