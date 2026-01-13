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
let params = UpdateFeedPostPublicParams {
    tenant_id: "acme-corp-tenant".to_string(),
    post_id: "news/article-12345".to_string(),
    update_feed_post_params: models::UpdateFeedPostParams {
        title: Some("Acme launches unified search".to_string()),
        content: Some("Acme Corp released a unified search feature that improves discovery across products.".to_string()),
        links: Some(vec![
            models::FeedPostLink { url: "https://acme.com/blog/unified-search".to_string(), title: Some("Read the announcement".to_string()) }
        ]),
        media: Some(vec![
            models::FeedPostMediaItem {
                id: Some("media-1122".to_string()),
                caption: Some("Feature screenshot".to_string()),
                assets: Some(vec![
                    models::FeedPostMediaItemAsset { url: "https://cdn.acme.com/images/feature-1122.jpg".to_string(), mime_type: Some("image/jpeg".to_string()) }
                ])
            }
        ]),
        is_published: Some(true),
    },
    broadcast_id: Some("broadcast-789".to_string()),
    sso: Some("sso-token-abc".to_string()),
};
let response: CreateFeedPostPublic200Response = update_feed_post_public(&configuration, params).await?;
[inline-code-end]
