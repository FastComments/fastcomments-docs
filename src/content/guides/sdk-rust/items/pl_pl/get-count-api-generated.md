## Parametry

| Name | Type | Wymagane | Opis |
|------|------|----------|-------------|
| text_search | String | Nie |  |
| by_ip_from_comment | String | Nie |  |
| filter | String | Nie |  |
| search_filters | String | Nie |  |
| demo | bool | Nie |  |
| sso | String | Nie |  |

## Odpowiedź

Zwraca: [`ModerationApiCountCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_count_comments_response.rs)

## Przykład

[inline-code-attrs-start title = 'get_count Przykład'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---