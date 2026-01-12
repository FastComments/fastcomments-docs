## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_ids | String | Yes |  |
| sso | String | No |  |

## Response

Returns: [`CheckedCommentsForBlocked200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/checked_comments_for_blocked_200_response.rs)

## Example

[inline-code-attrs-start title = 'checked_comments_for_blocked Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_checked_comments() -> Result<CheckedCommentsForBlocked200Response, Error> {
    let config: configuration::Configuration = configuration::Configuration::default();
    let params: CheckedCommentsForBlockedParams = CheckedCommentsForBlockedParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_ids: "cmt-12345,cmt-67890".to_string(),
        sso: Some("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example_payload".to_string()),
    };
    let response: CheckedCommentsForBlocked200Response =
        checked_comments_for_blocked(&config, params).await?;
    Ok(response)
}
[inline-code-end]
