## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| comment_id | String | כן |  |
| url_id | String | כן |  |
| broadcast_id | String | כן |  |
| vote_body_params | models::VoteBodyParams | כן |  |
| session_id | String | לא |  |
| sso | String | לא |  |

## תגובה

מחזיר: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_comment_200_response.rs)

---