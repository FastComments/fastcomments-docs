---
## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| text_search | String | 아니오 |  |
| by_ip_from_comment | String | 아니오 |  |
| filters | String | 아니오 |  |
| search_filters | String | 아니오 |  |
| sorts | String | 아니오 |  |
| sso | String | 아니오 |  |

## 응답

반환: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_export_response.rs)

## 예제

[inline-code-attrs-start title = 'post_api_export 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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