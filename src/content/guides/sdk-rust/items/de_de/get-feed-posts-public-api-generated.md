req
tenantId
afterId

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| after_id | String | Nein |  |
| limit | i32 | Nein |  |
| tags | Vec<String> | Nein |  |
| sso | String | Nein |  |
| is_crawler | bool | Nein |  |
| include_user_info | bool | Nein |  |

## Antwort

Gibt zurück: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_public_200_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_feed_posts_public Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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