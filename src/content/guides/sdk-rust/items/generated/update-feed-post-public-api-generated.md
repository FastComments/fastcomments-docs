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
let params: UpdateFeedPostPublicParams = UpdateFeedPostPublicParams {
    tenant_id: "acme-corp-tenant".to_string(),
    post_id: "news/article-2025-11-21".to_string(),
    update_feed_post_params: models::UpdateFeedPostParams {
        title: Some("Global Markets Rally After Policy Update".to_string()),
        body: Some("Markets responded positively to the central bank announcement...".to_string()),
        tags: Some(vec!["markets".to_string(), "economy".to_string()]),
        media: Some(vec![
            models::FeedPostMediaItem {
                id: Some("img-hero-01".to_string()),
                kind: Some("image".to_string()),
                assets: Some(vec![
                    models::FeedPostMediaItemAsset {
                        url: "https://cdn.acme-corp.com/news/hero-2025.jpg".to_string(),
                        mime_type: Some("image/jpeg".to_string())
                    }
                ])
            }
        ]),
        links: Some(vec![
            models::FeedPostLink {
                url: "https://acme-corp.com/deep-dive".to_string(),
                title: Some("Full analysis".to_string())
            }
        ]),
        ..Default::default()
    },
    broadcast_id: Some("broadcast-789".to_string()),
    sso: Some("sso-token-abc123".to_string())
};
let response: CreateFeedPostPublic200Response = update_feed_post_public(&configuration, params).await?;
[inline-code-end]
