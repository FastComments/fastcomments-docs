## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|-------------|-----------|
| tenant_id | String | Да |  |
| page | f64 | Нет |  |
| count | f64 | Нет |  |
| text_search | String | Нет |  |
| by_ip_from_comment | String | Нет |  |
| filters | String | Нет |  |
| search_filters | String | Нет |  |
| sorts | String | Нет |  |
| demo | bool | Нет |  |
| sso | String | Нет |  |

## Ответ

Возвращает: [`ModerationApiGetCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_get_comments_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример get_api_comments'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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