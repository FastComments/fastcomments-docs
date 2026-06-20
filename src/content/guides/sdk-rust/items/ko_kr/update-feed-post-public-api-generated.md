## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| post_id | String | 예 |  |
| update_feed_post_params | models::UpdateFeedPostParams | 예 |  |
| broadcast_id | String | 아니오 |  |
| sso | String | 아니오 |  |

## 응답

반환: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_response.rs)

## 예제

[inline-code-attrs-start title = 'update_feed_post_public 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: UpdateFeedPostPublicParams = UpdateFeedPostPublicParams {
    tenant_id: "acme-corp-tenant".to_string(),
    post_id: "news/product-launch-2026".to_string(),
    update_feed_post_params: models::UpdateFeedPostParams {
        title: "Acme Product Launch".to_string(),
        content: "Acme releases version 2.0 with major performance and security improvements.".to_string(),
        media: Some(vec![
            models::FeedPostMediaItem {
                asset: models::FeedPostMediaItemAsset {
                    url: "https://cdn.acme.com/images/product-v2.jpg".to_string(),
                    mime_type: Some("image/jpeg".to_string())
                }
            }
        ]),
        links: Some(vec![
            models::FeedPostLink {
                url: "https://acme.com/blog/product-v2".to_string(),
                title: Some("Product v2 announcement".to_string())
            }
        ]),
        ..Default::default()
    },
    broadcast_id: Some("broadcast-789".to_string()),
    sso: Some("sso-token-eyJhbGciOiJIUzI1Ni".to_string()),
};
let response: CreateFeedPostResponse = update_feed_post_public(&configuration, params).await?;
[inline-code-end]