req
tenantId
afterId

## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|-------------|----------|
| tenant_id | String | Да |  |
| after_id | String | Нет |  |
| limit | i32 | Нет |  |
| tags | Vec<String> | Нет |  |
| sso | String | Нет |  |
| is_crawler | bool | Нет |  |
| include_user_info | bool | Нет |  |

## Ответ

Возвращает: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/public_feed_posts_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример get_feed_posts_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetFeedPostsPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        after_id: Some("post123".to_string()),
        limit: Some(20),
        tags: Some(vec!["news".to_string(), "article".to_string()]),
        sso: Some("sso-token-xyz".to_string()),
        is_crawler: Some(false),
        include_user_info: Some(true),
    };
    let _response = get_feed_posts_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---