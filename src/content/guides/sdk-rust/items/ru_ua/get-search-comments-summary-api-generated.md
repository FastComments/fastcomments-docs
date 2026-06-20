## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| value | String | Нет |  |
| filters | String | Нет |  |
| search_filters | String | Нет |  |
| sso | String | Нет |  |

## Ответ

Возвращает: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_comment_search_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример get_search_comments_summary'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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