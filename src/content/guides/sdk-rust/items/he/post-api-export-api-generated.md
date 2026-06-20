## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| text_search | String | לא |  |
| by_ip_from_comment | String | לא |  |
| filters | String | לא |  |
| search_filters | String | לא |  |
| sorts | String | לא |  |
| sso | String | לא |  |

## תגובה

מחזיר: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_export_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-post_api_export'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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