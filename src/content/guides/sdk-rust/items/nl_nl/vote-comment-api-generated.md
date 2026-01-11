## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| comment_id | String | Ja |  |
| url_id | String | Ja |  |
| broadcast_id | String | Ja |  |
| vote_body_params | models::VoteBodyParams | Ja |  |
| session_id | String | Nee |  |
| sso | String | Nee |  |

## Respons

Geeft terug: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_comment_200_response.rs)

---