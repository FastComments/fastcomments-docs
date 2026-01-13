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
let create_feed_post_params: models::CreateFeedPostParams = models::CreateFeedPostParams {
    title: "Major outage impacting Acme services".to_string(),
    body: "We're investigating a major outage affecting login and API traffic.".to_string(),
    author_name: Some("Acme Status Team".to_string()),
    media: Some(vec![
        models::FeedPostMediaItem {
            id: Some("media-001".to_string()),
            assets: Some(vec![
                models::FeedPostMediaItemAsset {
                    url: "https://cdn.acme.com/outage/overview.jpg".to_string(),
                    mime_type: Some("image/jpeg".to_string()),
                    alt_text: Some("Overview diagram".to_string()),
                }
            ]),
        }
    ]),
    links: Some(vec![
        models::FeedPostLink { url: "https://status.acme.com/incident/456".to_string(), title: Some("Incident details".to_string()) }
    ]),
};

let params: CreateFeedPostPublicParams = CreateFeedPostPublicParams {
    tenant_id: "acme-corp-tenant".to_string(),
    create_feed_post_params,
    broadcast_id: Some("broadcast-20260112".to_string()),
    sso: Some("sso-secure-token-xyz".to_string()),
};

let response: CreateFeedPostPublic200Response = create_feed_post_public(&configuration, params).await?;
[inline-code-end]
