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
async fn run() -> Result<(), Error> {
    let params: VoteCommentParams = VoteCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-12345".to_string(),
        url_id: "news/politics/2026-election".to_string(),
        broadcast_id: "broadcast-nytimes-001".to_string(),
        vote_body_params: models::VoteBodyParams { ..Default::default() },
        session_id: Some("sess-9f8e7d".to_string()),
        sso: Some("user-42@example.com".to_string()),
    };
    let response: VoteComment200Response = vote_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
