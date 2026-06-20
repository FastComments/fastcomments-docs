## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| create_feed_post_params | models::CreateFeedPostParams | Da |  |
| broadcast_id | String | Ne |  |
| sso | String | Ne |  |

## Odgovor

Vrača: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer create_feed_post_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<CreateFeedPostResponse, Error> {
    let params: CreateFeedPostPublicParams = CreateFeedPostPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_feed_post_params: models::CreateFeedPostParams {
            title: "Acme Product Launch".to_string(),
            content: "We're excited to launch our new product line today.".to_string(),
            path: "news/product-launch".to_string(),
            media: vec![models::FeedPostMediaItem {
                asset: models::FeedPostMediaItemAsset {
                    url: "https://cdn.acme.com/images/launch.jpg".to_string(),
                    mime_type: Some("image/jpeg".to_string()),
                },
                caption: Some("Launch hero image".to_string()),
            }],
            ..Default::default()
        },
        broadcast_id: Some("broadcast-2026-06".to_string()),
        sso: Some("sso-user-jane-xyz".to_string()),
    };
    let response: CreateFeedPostResponse = create_feed_post_public(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]