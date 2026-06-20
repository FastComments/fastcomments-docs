## Параметри

| Назва | Тип | Обов'язкове | Опис |
|------|------|----------|-------------|
| value | String | Ні |  |
| filters | String | Ні |  |
| search_filters | String | Ні |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_comment_search_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад get_search_comments_summary'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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