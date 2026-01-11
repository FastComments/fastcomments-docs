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
let params: CreateFeedPostParams = CreateFeedPostParams {
    tenant_id: "acme-corp-tenant".to_string(),
    create_feed_post_params: models::CreateFeedPostParams {
        title: "Major Outage Affecting Metro Area".to_string(),
        body: "We are investigating reports of an outage impacting internet and voice services across the metro area. Engineers are on site and updates will follow.".to_string(),
        author_id: "newsroom-ops".to_string(),
        media_items: vec![
            FeedPostMediaItem {
                asset: FeedPostMediaItemAsset {
                    url: "https://cdn.example.com/images/outage-map.png".to_string(),
                    mime_type: "image/png".to_string()
                },
                caption: Some("Affected neighborhoods".to_string())
            }
        ],
        links: vec![
            FeedPostLink {
                url: "https://status.example.com/incidents/2025-11-21".to_string(),
                title: Some("Incident details".to_string())
            }
        ],
        tags: vec!["outage".to_string(), "status".to_string()],
        allow_comments: Some(true)
    },
    broadcast_id: Some("live-feed-2025-11-21".to_string()),
    is_live: Some(true),
    do_spam_check: Some(true),
    skip_dup_check: Some(false)
};
let response: CreateFeedPost200Response = create_feed_post(&configuration, params).await?;
[inline-code-end]
