## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Da |  |
| post_ids | Vec<String> | Da |  |
| sso | String | Ne |  |

## Odgovor

Vraća: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/feed_posts_stats_response.rs)

## Primjer

[inline-code-attrs-start title = 'Primjer get_feed_posts_stats'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_feed_stats() -> Result<(), Error> {
    let params = GetFeedPostsStatsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        post_ids: vec![
            "news/article/123".to_string(),
            "blog/post/456".to_string(),
        ],
        sso: Some("sso-token-xyz".to_string()),
    };
    let _response = get_feed_posts_stats(&configuration, params).await?;
    Ok(())
}
[inline-code-end]