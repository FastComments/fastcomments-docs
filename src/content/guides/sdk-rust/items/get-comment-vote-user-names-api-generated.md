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
async fn example(config: &configuration::Configuration) -> Result<GetCommentVoteUserNames200Response, Error> {
    let params: GetCommentVoteUserNamesParams = GetCommentVoteUserNamesParams {
        tenant_id: "acme-news-tenant".to_string(),
        comment_id: "news/article-2026-01-13-breaking-update-67890".to_string(),
        dir: 1,
        sso: Some("sso-token-user-9a8b7c".to_string()),
    };
    let response: GetCommentVoteUserNames200Response = get_comment_vote_user_names(config, params).await?;
    Ok(response)
}
[inline-code-end]
