req
tenantId
afterId

## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| after_id | String | Non |  |
| limit | i32 | Non |  |
| tags | Vec<String> | Non |  |
| sso | String | Non |  |
| is_crawler | bool | Non |  |
| include_user_info | bool | Non |  |

## Réponse

Renvoie : [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_public_200_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_feed_posts_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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