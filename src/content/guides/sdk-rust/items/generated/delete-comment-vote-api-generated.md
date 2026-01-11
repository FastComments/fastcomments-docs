## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| vote_id | String | Yes |  |
| url_id | String | Yes |  |
| broadcast_id | String | Yes |  |
| edit_key | String | No |  |
| sso | String | No |  |

## Response

Returns: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_vote_200_response.rs)

## Example

[inline-code-attrs-start title = 'delete_comment_vote Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete_vote() -> Result<(), Error> {
    let params: DeleteCommentVoteParams = DeleteCommentVoteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-7f3a2b".to_string(),
        vote_id: "vote-4d2c1a".to_string(),
        url_id: "news/article/major-update-2025".to_string(),
        broadcast_id: "broadcast-2025-11-21".to_string(),
        edit_key: Some("edk-9b8c7d".to_string()),
        sso: Some("sso-user-xyz123".to_string()),
    };
    let response: DeleteCommentVote200Response = delete_comment_vote(configuration, params).await?;
    Ok(())
}
[inline-code-end]
