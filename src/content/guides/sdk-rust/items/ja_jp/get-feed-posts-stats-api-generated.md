## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| post_ids | Vec<String> | はい |  |
| sso | String | いいえ |  |

## レスポンス

戻り値: [`GetFeedPostsStats200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_stats_200_response.rs)

## 例

[inline-code-attrs-start title = 'get_feed_posts_stats の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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