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
let vote_body: models::VoteBodyParams = models::VoteBodyParams {
    vote: "upvote".to_string(),
};

let params: VoteCommentParams = VoteCommentParams {
    tenant_id: "acme-corp-tenant".to_string(),
    comment_id: "cmt-9876543".to_string(),
    url_id: "news/article/2026/01/13/product-launch".to_string(),
    broadcast_id: "web-default".to_string(),
    vote_body_params: vote_body,
    session_id: Some("sess-4f3b2a9c".to_string()),
    sso: Some("sso-jwt-abcdef123456".to_string()),
};

let response: VoteComment200Response = vote_comment(&configuration, params).await?;
[inline-code-end]
