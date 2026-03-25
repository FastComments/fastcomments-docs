## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| post_id | String | Ναι |  |
| update_feed_post_params | models::UpdateFeedPostParams | Ναι |  |
| broadcast_id | String | Όχι |  |
| sso | String | Όχι |  |

## Απόκριση

Επιστρέφει: [`CreateFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_public_200_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα update_feed_post_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: UpdateFeedPostPublicParams = UpdateFeedPostPublicParams {
    tenant_id: "acme-corp-tenant".to_string(),
    post_id: "news/article-2026-03-25".to_string(),
    update_feed_post_params: models::UpdateFeedPostParams {
        title: "Acme Widget Launch".to_string(),
        body: "Acme today launched the next-generation Widget with improved performance and battery life.".to_string(),
        tags: vec!["product".to_string(), "launch".to_string()],
        media: vec![
            models::FeedPostMediaItem {
                url: "https://cdn.acme.com/images/widget-launch.jpg".to_string(),
                asset: Some(models::FeedPostMediaItemAsset {
                    mime_type: "image/jpeg".to_string(),
                    size: Some(142000),
                }),
            }
        ],
        links: vec![
            models::FeedPostLink {
                url: "https://acme.com/blog/widget-launch".to_string(),
                title: Some("Read the full announcement".to_string()),
            }
        ],
    },
    broadcast_id: Some("broadcast-2026-03".to_string()),
    sso: Some("sso-token-9f8e7d".to_string()),
};

let response: CreateFeedPostPublic200Response = update_feed_post_public(configuration, params).await?;
[inline-code-end]