## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| post_ids | Vec<String> | 예 |  |
| sso | String | 아니요 |  |

## 응답

반환: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/feed_posts_stats_response.rs)

## 예제

[inline-code-attrs-start title = 'get_feed_posts_stats 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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
    let _api_status: ApiStatus = /* 필요에 따라 stats를 사용 */ stats.into();
    Ok(())
}
[inline-code-end]

---