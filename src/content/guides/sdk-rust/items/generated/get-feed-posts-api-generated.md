
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
async fn run() -> Result<GetFeedPosts200Response, Error> {
    let params: GetFeedPostsParams = GetFeedPostsParams {
        tenant_id: String::from("acme-corp-tenant"),
        after_id: Some(String::from("post_987654321")),
        limit: Some(25),
        tags: Some(vec![String::from("news/article"), String::from("product-announcements")]),
    };
    let response: GetFeedPosts200Response = get_feed_posts(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
