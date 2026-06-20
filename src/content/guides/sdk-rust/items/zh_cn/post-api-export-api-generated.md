## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|-------------|
| text_search | String | 否 |  |
| by_ip_from_comment | String | 否 |  |
| filters | String | 否 |  |
| search_filters | String | 否 |  |
| sorts | String | 否 |  |
| sso | String | 否 |  |

## 响应

返回：[`ModerationExportResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_export_response.rs)

## 示例

[inline-code-attrs-start title = 'post_api_export 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---