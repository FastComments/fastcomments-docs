## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| dir | i32 | Yes |  |
| sso | String | No |  |

## Response

Returns: [`GetCommentVoteUserNames200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_vote_user_names_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_comment_vote_user_names Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_vote_users() -> Result<GetCommentVoteUserNames200Response, Error> {
    let params: GetCommentVoteUserNamesParams = GetCommentVoteUserNamesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article/12345".to_string(),
        dir: 1,
        sso: Some("sso-token-abc123".to_string()),
    };
    let response: GetCommentVoteUserNames200Response = get_comment_vote_user_names(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
