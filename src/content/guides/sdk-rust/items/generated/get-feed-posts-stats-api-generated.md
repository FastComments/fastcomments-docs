## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| post_ids | Vec<String> | Yes |  |
| sso | String | No |  |

## Response

Returns: [`GetFeedPostsStats200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_stats_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_feed_posts_stats Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<GetFeedPostsStats200Response, Error> {
    let params: GetFeedPostsStatsParams = GetFeedPostsStatsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        post_ids: vec![
            "news/article-2025-11-01".to_string(),
            "tech/review-2025-10-20".to_string(),
        ],
        sso: Some("sso-session-token-9f3b".to_string()),
    };
    let stats: GetFeedPostsStats200Response = get_feed_posts_stats(&configuration, params).await?;
    Ok(stats)
}
[inline-code-end]
