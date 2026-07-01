## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| sso | String | No |  |

## Response

Returns: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_ban_status_response.rs)

## Example

[inline-code-attrs-start title = 'get_comment_ban_status Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetCommentBanStatusParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "comment-12345".to_string(),
        sso: Some("user@example.com".to_string()),
    };
    let _response: GetCommentBanStatusResponse = get_comment_ban_status(config, params).await?;
    Ok(())
}
[inline-code-end]
