## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|------|-------------|
| tenant_id | String | 是 |  |
| post_ids | Vec<String> | 是 |  |
| sso | String | 否 |  |

## 回應

回傳: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/feed_posts_stats_response.rs)

## 範例

[inline-code-attrs-start title = 'get_feed_posts_stats 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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
    let _api_status: ApiStatus = /* 根據需要使用 stats */ stats.into();
    Ok(())
}
[inline-code-end]

---