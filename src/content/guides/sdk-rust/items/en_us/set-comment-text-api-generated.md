## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| broadcast_id | String | Yes |  |
| comment_text_update_request | models::CommentTextUpdateRequest | Yes |  |
| edit_key | String | No |  |
| sso | String | No |  |

## Response

Returns: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_text_200_response.rs)