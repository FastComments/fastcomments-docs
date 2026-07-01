req
tenantId
afterId

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenant_id | String | Ja |  |
| after_id | String | Nein |  |
| limit | i32 | Nein |  |
| tags | Vec<String> | Nein |  |
| sso | String | Nein |  |
| is_crawler | bool | Nein |  |
| include_user_info | bool | Nein |  |

## Antwort

Rückgabe: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/public_feed_posts_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_feed_posts_public Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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