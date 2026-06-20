## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| text_search | String | Ne |  |
| by_ip_from_comment | String | Ne |  |
| filter | String | Ne |  |
| search_filters | String | Ne |  |
| demo | bool | Ne |  |
| sso | String | Ne |  |

## Odgovor

Vraća: [`ModerationApiCountCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_count_comments_response.rs)

## Primjer

[inline-code-attrs-start title = 'get_count Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_count() -> Result<ModerationApiCountCommentsResponse, Error> {
    let params: GetCountParams = GetCountParams {
        text_search: Some("breaking election coverage".to_string()),
        by_ip_from_comment: Some("203.0.113.45".to_string()),
        filter: Some("status:approved".to_string()),
        search_filters: Some("section:politics tag:analysis".to_string()),
        demo: Some(false),
        sso: Some("acme-corp-tenant".to_string()),
    };
    let response: ModerationApiCountCommentsResponse = get_count(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]