## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| text_search | String | Ні |  |
| by_ip_from_comment | String | Ні |  |
| filter | String | Ні |  |
| search_filters | String | Ні |  |
| demo | bool | Ні |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`ModerationApiCountCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_count_comments_response.rs)

## Приклад

[inline-code-attrs-start title = 'get_count Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        text_search: Some("breaking news".to_string()),
        by_ip_from_comment: Some("192.168.1.1".to_string()),
        filter: Some("status:approved".to_string()),
        search_filters: Some("author:john".to_string()),
        demo: Some(false),
        sso: Some("sso-token-123".to_string()),
    };
    let _response = get_count(&configuration, params).await?;
    Ok(())
}
[inline-code-end]