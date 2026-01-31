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
async fn run() -> Result<(), Error> {
    let params: CreateFeedPostParams = CreateFeedPostParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_feed_post_params: models::CreateFeedPostParams {
            title: "Acme and Globex Announce Strategic Merger".to_string(),
            body: "Acme Corporation and Globex have entered into a definitive agreement to merge operations effective Q2 2026.".to_string(),
            tags: Some(vec!["business".to_string(), "merger".to_string(), "press-release".to_string()]),
            media: Some(vec![
                FeedPostMediaItem {
                    url: "https://cdn.acme.com/images/merger.jpg".to_string(),
                    assets: Some(vec![
                        FeedPostMediaItemAsset {
                            url: "https://cdn.acme.com/images/merger_1200.jpg".to_string(),
                            width: Some(1200),
                            height: Some(800),
                        }
                    ]),
                }
            ]),
            links: Some(vec![
                FeedPostLink {
                    url: "https://acme.com/press/merger".to_string(),
                    title: Some("Read the full press release".to_string()),
                }
            ]),
            ..Default::default()
        },
        broadcast_id: Some("corporate-newsletter-2026-01".to_string()),
        is_live: Some(false),
        do_spam_check: Some(true),
        skip_dup_check: Some(false),
    };
    let response: CreateFeedPost200Response = create_feed_post(&configuration, params).await?;
    let _response = response;
    Ok(())
}
[inline-code-end]
