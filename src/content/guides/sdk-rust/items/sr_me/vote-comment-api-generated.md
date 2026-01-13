## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| comment_id | String | Да |  |
| url_id | String | Да |  |
| broadcast_id | String | Да |  |
| vote_body_params | models::VoteBodyParams | Да |  |
| session_id | String | Не |  |
| sso | String | Не |  |

## Одговор

Враћа: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_comment_200_response.rs)

---