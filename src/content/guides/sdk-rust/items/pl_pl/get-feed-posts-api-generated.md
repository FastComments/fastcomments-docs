żądanie
tenantId
afterId

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| after_id | String | Nie |  |
| limit | i32 | Nie |  |
| tags | Vec<String> | Nie |  |

## Odpowiedź

Zwraca: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład get_feed_posts'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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