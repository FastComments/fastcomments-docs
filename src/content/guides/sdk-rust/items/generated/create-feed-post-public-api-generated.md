## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_feed_post_params | models::CreateFeedPostParams | Yes |  |
| broadcast_id | String | No |  |
| sso | String | No |  |

## Response

Returns: [`CreateFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'create_feed_post_public Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: CreateFeedPostPublicParams = CreateFeedPostPublicParams {
    tenant_id: "acme-corp-tenant".to_string(),
    create_feed_post_params: models::CreateFeedPostParams {
        title: "Acme One Launch".to_string(),
        body: "Acme Corp announces the Acme One device, shipping worldwide next month.".to_string(),
        author_id: "editor-42".to_string(),
        media: Some(vec![models::FeedPostMediaItem {
            asset: Some(models::FeedPostMediaItemAsset {
                url: "https://cdn.acme.com/images/acme-one.jpg".to_string(),
                mime_type: Some("image/jpeg".to_string())
            }),
            alt_text: Some("Acme One product image".to_string())
        }]),
        links: Some(vec![models::FeedPostLink {
            url: "https://acme.com/blog/acme-one-launch".to_string(),
            title: Some("Read the announcement".to_string())
        }]),
        tags: Some(vec!["product".to_string(), "launch".to_string()]),
        is_public: Some(true)
    },
    broadcast_id: Some("broadcast-20251121".to_string()),
    sso: Some("sso-token-xyz-789".to_string()),
};
let response: CreateFeedPostPublic200Response = create_feed_post_public(&configuration, params).await?;
[inline-code-end]
