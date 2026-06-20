## Parametri

| Name | Type | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| post_ids | Vec<String> | Da |  |
| sso | String | Ne |  |

## Odgovor

Vrne: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/feed_posts_stats_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer get_feed_posts_stats'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_main(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params: GetFeedPostsStatsParams = GetFeedPostsStatsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        post_ids: vec![
            "news/2026-product-launch".to_string(),
            "blog/engineering-architecture".to_string()
        ],
        sso: Some("sso.jwt.token.eyJhbGci...".to_string()),
    };
    let stats: FeedPostsStatsResponse = get_feed_posts_stats(configuration, params).await?;
    let _api_status: ApiStatus = /* uporabite stats po potrebi */ stats.into();
    Ok(())
}
[inline-code-end]