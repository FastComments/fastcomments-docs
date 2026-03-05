## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_feed_post_params | models::CreateFeedPostParams | Yes |  |
| broadcast_id | String | No |  |
| is_live | bool | No |  |
| do_spam_check | bool | No |  |
| skip_dup_check | bool | No |  |

## Response

Returns: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_200_response.rs)

## Example

[inline-code-attrs-start title = 'create_feed_post Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_create_post() -> Result<CreateFeedPost200Response, Error> {
    let params: CreateFeedPostParams = CreateFeedPostParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_feed_post_params: models::CreateFeedPostParams {
            title: "Breaking: Rust 2.0 Released".to_string(),
            content: "Rust 2.0 brings performance and ergonomics improvements.".to_string(),
            author_id: "editor-123".to_string(),
            media: vec![
                models::FeedPostMediaItem {
                    kind: "image".to_string(),
                    assets: vec![
                        models::FeedPostMediaItemAsset {
                            url: "https://cdn.acme.com/images/rust2.png".to_string(),
                            mime_type: Some("image/png".to_string()),
                        }
                    ],
                }
            ],
            links: vec![
                models::FeedPostLink {
                    url: "https://acme.news/rust-2".to_string(),
                    title: Some("Full article".to_string()),
                }
            ],
            tags: vec!["rust".to_string(), "release".to_string()],
            visibility: Some("public".to_string()),
        },
        broadcast_id: Some("global-feed".to_string()),
        is_live: Some(true),
        do_spam_check: Some(true),
        skip_dup_check: Some(false),
    };
    let response: CreateFeedPost200Response = create_feed_post(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
