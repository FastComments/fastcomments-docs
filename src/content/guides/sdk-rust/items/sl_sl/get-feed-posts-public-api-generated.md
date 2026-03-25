req
tenantId
afterId

## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| after_id | String | Ne |  |
| limit | i32 | Ne |  |
| tags | Vec<String> | Ne |  |
| sso | String | Ne |  |
| is_crawler | bool | Ne |  |
| include_user_info | bool | Ne |  |

## Odgovor

Vrne: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_public_200_response.rs)

## Primer

[inline-code-attrs-start title = 'get_feed_posts_public Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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