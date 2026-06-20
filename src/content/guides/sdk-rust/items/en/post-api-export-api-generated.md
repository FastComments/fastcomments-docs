## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| text_search | String | No |  |
| by_ip_from_comment | String | No |  |
| filters | String | No |  |
| search_filters | String | No |  |
| sorts | String | No |  |
| sso | String | No |  |

## Response

Returns: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_export_response.rs)

## Example

[inline-code-attrs-start title = 'post_api_export Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_export() -> Result<ModerationExportResponse, Error> {
    let params: PostApiExportParams = PostApiExportParams {
        text_search: Some("climate policy debate".to_string()),
        by_ip_from_comment: Some("203.0.113.5".to_string()),
        filters: Some(r#"{"status":"approved","channel":"news/article"}"#.to_string()),
        search_filters: Some("created_after:2024-01-01".to_string()),
        sorts: Some("created_at:desc".to_string()),
        sso: Some("acme-corp-tenant".to_string()),
    };
    let export_response: ModerationExportResponse = post_api_export(&configuration, params).await?;
    Ok(export_response)
}
[inline-code-end]
