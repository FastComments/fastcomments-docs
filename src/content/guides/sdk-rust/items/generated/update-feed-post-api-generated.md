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
async fn run_update() -> Result<(), Error> {
    let feed_post: models::FeedPost = models::FeedPost {
        id: Some("article-12345".to_string()),
        title: Some("Acme Launches Next-Gen Widget".to_string()),
        body: Some("Acme announces its next-generation widget with improved battery life and AI features.".to_string()),
        author_id: Some("editor_jane".to_string()),
        tags: Some(vec!["product".to_string(), "announcement".to_string()]),
        published: Some(true),
        media: Some(vec![
            models::FeedPostMediaItem {
                asset: Some(models::FeedPostMediaItemAsset { url: Some("https://cdn.acme.com/images/widget.jpg".to_string()) }),
                media_type: Some("image".to_string()),
                caption: Some("Official product image".to_string())
            }
        ]),
        links: Some(vec![
            models::FeedPostLink { url: "https://acme.com/blog/widget".to_string(), title: Some("Read the full story".to_string()) }
        ])
    };
    let params: UpdateFeedPostParams = UpdateFeedPostParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article-12345".to_string(),
        feed_post
    };
    let _response: FlagCommentPublic200Response = update_feed_post(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
