## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| post_ids | Vec<String> | Yes |  |
| sso | String | No |  |

## 응답

Returns: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/feed_posts_stats_response.rs)

## 예시

[inline-code-attrs-start title = 'get_feed_posts_stats 예시'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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