req
tenantId
afterId

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| after_id | String | Ні |  |
| limit | i32 | Ні |  |
| tags | Vec<String> | Ні |  |
| sso | String | Ні |  |
| is_crawler | bool | Ні |  |
| include_user_info | bool | Ні |  |

## Відповідь

Повертає: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_public_200_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад get_feed_posts_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<GetFeedPostsPublic200Response, Error> {
    let params: GetFeedPostsPublicParams = GetFeedPostsPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        after_id: Some("post_987654".to_string()),
        limit: Some(25),
        tags: Some(vec!["news".to_string(), "technology".to_string()]),
        sso: Some("sso-token-abc123".to_string()),
        is_crawler: Some(false),
        include_user_info: Some(true),
    };
    let response: GetFeedPostsPublic200Response = get_feed_posts_public(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---