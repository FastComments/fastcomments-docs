req
tenantId
afterId

## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| after_id | String | Nie |  |
| limit | i32 | Nie |  |
| tags | Vec<String> | Nie |  |
| sso | String | Nie |  |
| is_crawler | bool | Nie |  |
| include_user_info | bool | Nie |  |

## Odpowiedź

Zwraca: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/public_feed_posts_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład get_feed_posts_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example() -> Result<(), Error> {
    let params: GetFeedPostsPublicParams = GetFeedPostsPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        after_id: Some("post_9f8d7c".to_string()),
        limit: Some(20),
        tags: Some(vec!["news".to_string(), "product-updates".to_string()]),
        sso: Some("sso-token-9a8b7c".to_string()),
        is_crawler: Some(false),
        include_user_info: Some(true),
    };
    let response: PublicFeedPostsResponse = get_feed_posts_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---