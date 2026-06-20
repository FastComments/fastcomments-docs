## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| page | f64 | Не |  |
| count | f64 | Не |  |
| text_search | String | Не |  |
| by_ip_from_comment | String | Не |  |
| filters | String | Не |  |
| search_filters | String | Не |  |
| sorts | String | Не |  |
| demo | bool | Не |  |
| sso | String | Не |  |

## Одговор

Враћа: [`ModerationApiGetCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_get_comments_response.rs)

## Пример

[inline-code-attrs-start title = 'get_api_comments Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params: GetApiCommentsParams = GetApiCommentsParams {
        page: Some(1.0),
        count: Some(20.0),
        text_search: Some(String::from("breaking election results")),
        by_ip_from_comment: Some(String::from("203.0.113.45")),
        filters: Some(String::from("status:approved,thread:news/article")),
        search_filters: Some(String::from("author:john.doe@example.com")),
        sorts: Some(String::from("-created_at")),
        demo: Some(false),
        sso: Some(String::from("acme-corp-tenant")),
    };
    let response: ModerationApiGetCommentsResponse = get_api_comments(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---