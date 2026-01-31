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
async fn run_update_feed_post() -> Result<(), Error> {
    let params: UpdateFeedPostParams = UpdateFeedPostParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article-2026-01-13".to_string(),
        feed_post: models::FeedPost { title: Some("Rust 1.72 Released".to_string()), content: Some("Improved async performance.".to_string()), author_id: Some("editor-jane".to_string()), tags: Some(vec!["rust".to_string(), "release".to_string()]), media: Some(vec![ models::FeedPostMediaItem { r#type: Some("image".to_string()), assets: Some(vec![ models::FeedPostMediaItemAsset { url: Some("https://cdn.acme.com/rust-1-72.jpg".to_string()), width: Some(1200), height: Some(630) } ]), caption: Some("Release banner".to_string()) } ]), links: Some(vec![ models::FeedPostLink { title: Some("Release notes".to_string()), url: Some("https://blog.rust-lang.org/".to_string()) } ]), published_at: Some("2026-01-13T09:00:00Z".to_string()), pinned: Some(false) },
    };
    let response: FlagCommentPublic200Response = update_feed_post(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
