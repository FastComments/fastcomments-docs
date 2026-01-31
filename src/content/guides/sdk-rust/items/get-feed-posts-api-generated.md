
req
tenantId
afterId

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| after_id | String | No |  |
| limit | i32 | No |  |
| tags | Vec<String> | No |  |

## Response

Returns: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_feed_posts Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params: GetFeedPostsParams = GetFeedPostsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        after_id: Some("post_1024".to_string()),
        limit: Some(20_i32),
        tags: Some(vec!["news".to_string(), "announcements".to_string()]),
    };
    let response: GetFeedPosts200Response = get_feed_posts(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
