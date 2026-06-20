## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| page | f64 | Ні |  |
| count | f64 | Ні |  |
| text_search | String | Ні |  |
| by_ip_from_comment | String | Ні |  |
| filters | String | Ні |  |
| search_filters | String | Ні |  |
| sorts | String | Ні |  |
| demo | bool | Ні |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`ModerationApiGetCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_get_comments_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад get_api_comments'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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