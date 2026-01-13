## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|-------------|
| tenant_id | String | Ja |  |
| comment_id | String | Ja |  |
| url_id | String | Ja |  |
| broadcast_id | String | Ja |  |
| vote_body_params | models::VoteBodyParams | Ja |  |
| session_id | String | Nein |  |
| sso | String | Nein |  |

## Antwort

Gibt zurück: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_comment_200_response.rs)

---