## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| url_id | String | Yes |  |
| broadcast_id | String | Yes |  |
| vote_body_params | models::VoteBodyParams | Yes |  |
| session_id | String | No |  |
| sso | String | No |  |

## Response

Returns: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_comment_200_response.rs)
