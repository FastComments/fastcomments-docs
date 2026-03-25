req
tenantId
afterId

## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| after_id | String | Hayır |  |
| limit | i32 | Hayır |  |
| tags | Vec<String> | Hayır |  |

## Yanıt

Dönen Değer: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_200_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_feed_posts Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetFeedPostsParams = GetFeedPostsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        after_id: Some("post_98765".to_string()),
        limit: Some(20),
        tags: Some(vec!["news".to_string(), "technology".to_string()]),
    };
    let feed: GetFeedPosts200Response = get_feed_posts(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---