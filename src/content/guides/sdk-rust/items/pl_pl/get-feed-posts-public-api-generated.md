req
tenantId
afterId

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
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

[inline-code-attrs-start title = 'get_feed_posts_public Przykład'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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