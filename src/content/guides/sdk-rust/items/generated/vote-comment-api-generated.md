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

## Example

[inline-code-attrs-start title = 'vote_comment Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: VoteCommentParams = VoteCommentParams {
    tenant_id: "acme-corp-tenant".to_string(),
    comment_id: "cmt-20251121-84a".to_string(),
    url_id: "news/article/2025/11/21/major-update".to_string(),
    broadcast_id: "broadcast-2025-11-21-1".to_string(),
    vote_body_params: models::VoteBodyParams { vote: 1 },
    session_id: Some("sess-9f4b".to_string()),
    sso: Some("sso-jwt-token-xyz".to_string()),
};

let response: VoteComment200Response = vote_comment(&configuration, params).await?;
[inline-code-end]
