## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| text_search | String | No |  |
| by_ip_from_comment | String | No |  |
| filter | String | No |  |
| search_filters | String | No |  |
| demo | bool | No |  |
| sso | String | No |  |

## Response

Returns: [`ModerationApiCountCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_count_comments_response.rs)

## Example

[inline-code-attrs-start title = 'get_count Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        text_search: Some("breaking news".to_string()),
        by_ip_from_comment: Some("192.168.1.1".to_string()),
        filter: Some("status:approved".to_string()),
        search_filters: Some("author:john".to_string()),
        demo: Some(false),
        sso: Some("sso-token-123".to_string()),
    };
    let _response = get_count(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
