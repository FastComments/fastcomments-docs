## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| create_feed_post_params | models::CreateFeedPostParams | Так |  |
| broadcast_id | String | Ні |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_response.rs)

## Приклад

[inline-code-attrs-start title = 'create_feed_post_public Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---