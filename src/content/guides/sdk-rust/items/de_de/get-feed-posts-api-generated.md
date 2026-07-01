---
req
tenantId
afterId

## Parameter

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| after_id | String | Nein |  |
| limit | i32 | Nein |  |
| tags | Vec<String> | Nein |  |

## Antwort

Rückgabe: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_feed_posts Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_feed(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetFeedPostsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        after_id: Some("post-12345".to_string()),
        limit: Some(20),
        tags: Some(vec!["news".to_string(), "article".to_string()]),
    };
    let _response = get_feed_posts(config, params).await?;
    Ok(())
}
[inline-code-end]

---