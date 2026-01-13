## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| feed_post | models::FeedPost | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'update_feed_post Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<FlagCommentPublic200Response, Error> {
    let params: UpdateFeedPostParams = UpdateFeedPostParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article-2026-01-12-123".to_string(),
        feed_post: models::FeedPost {
            title: "Major Product Launch".to_string(),
            content: Some("Acme has launched its next-gen widget with improved performance and battery life.".to_string()),
            author_id: Some("editor-42".to_string()),
            links: Some(vec![models::FeedPostLink { url: "https://acme.com/product".to_string(), title: Some("Product page".to_string()) }]),
            media: Some(vec![models::FeedPostMediaItem { url: "https://cdn.acme.com/images/widget.jpg".to_string(), asset: Some(models::FeedPostMediaItemAsset { src: Some("https://cdn.acme.com/assets/widget.jpg".to_string()) }) }]),
            published: Some(true),
        },
    };

    let response: FlagCommentPublic200Response = update_feed_post(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
