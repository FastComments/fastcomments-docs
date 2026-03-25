## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| post_ids | Vec<String> | Oui |  |
| sso | String | Non |  |

## Réponse

Retourne: [`GetFeedPostsStats200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_stats_200_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_feed_posts_stats'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_feed_stats() -> Result<(), Error> {
    let params: GetFeedPostsStatsParams = GetFeedPostsStatsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        post_ids: vec![
            "news/article/2026/03/25/product-launch".to_string(),
            "blog/product-updates/q1-2026".to_string(),
        ],
        sso: Some("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.examplepayload.signature".to_string()),
    };
    let stats: GetFeedPostsStats200Response = get_feed_posts_stats(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---