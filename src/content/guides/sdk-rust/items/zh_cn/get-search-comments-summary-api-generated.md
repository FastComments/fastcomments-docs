## 参数

| 名称 | 类型 | 必需 | 说明 |
|------|------|------|------|
| value | String | 否 |  |
| filters | String | 否 |  |
| search_filters | String | 否 |  |
| sso | String | 否 |  |

## 响应

返回: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_comment_search_response.rs)

## 示例

[inline-code-attrs-start title = 'get_search_comments_summary 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_summary() -> Result<ModerationCommentSearchResponse, Error> {
    let params: GetSearchCommentsSummaryParams = GetSearchCommentsSummaryParams {
        value: Some("climate change".to_string()),
        filters: Some(r#"{"tenant":"acme-corp-tenant","stream":"news/article"}"#.to_string()),
        search_filters: Some(r#"{"author_email":"reporter@news.example.com","moderation_status":"reviewed"}"#.to_string()),
        sso: Some("sso:acme:user:67890".to_string()),
    };
    let summary: ModerationCommentSearchResponse = get_search_comments_summary(&configuration, params).await?;
    Ok(summary)
}
[inline-code-end]

---