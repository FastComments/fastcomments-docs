## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
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
async fn run() -> Result<(), Error> {
    let params = GetApiCommentsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        page: Some(1.0),
        count: Some(20.0),
        text_search: Some("rust".to_string()),
        by_ip_from_comment: None,
        filters: Some("status:approved".to_string()),
        search_filters: Some("author:john".to_string()),
        sorts: Some("date:desc".to_string()),
        demo: Some(false),
        sso: None,
    };
    let _response = get_api_comments(&configuration, params).await?;
    Ok(())
}
[inline-code-end]