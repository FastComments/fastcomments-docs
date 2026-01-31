## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| post_id | String | Yes |  |
| update_feed_post_params | models::UpdateFeedPostParams | Yes |  |
| broadcast_id | String | No |  |
| sso | String | No |  |

## Response

Returns: [`CreateFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'update_feed_post_public Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_update_post() -> Result<CreateFeedPostPublic200Response, Error> {
    let cfg: &configuration::Configuration = &configuration;
    let params: UpdateFeedPostPublicParams = UpdateFeedPostPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        post_id: "news/article-2026-launch".to_string(),
        update_feed_post_params: models::UpdateFeedPostParams {
            title: Some("Product Launch Announcement".to_string()),
            content: Some("<p>Acme is excited to announce our 2026 product line.</p>".to_string()),
            published: Some(true),
            media: Some(vec![models::FeedPostMediaItem {
                url: "https://cdn.acme.com/images/launch.jpg".to_string(),
                caption: Some("Product reveal at keynote".to_string()),
                assets: None,
            }]),
            links: Some(vec![models::FeedPostLink {
                url: "https://acme.com/blog/launch".to_string(),
                title: Some("Read the full announcement".to_string()),
            }]),
        },
        broadcast_id: Some("broadcast-2026-01".to_string()),
        sso: Some("sso-secure-token-abc123".to_string()),
    };
    let resp: CreateFeedPostPublic200Response = update_feed_post_public(cfg, params).await?;
    Ok(resp)
}
[inline-code-end]
