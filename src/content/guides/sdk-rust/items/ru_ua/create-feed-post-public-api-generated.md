## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| create_feed_post_params | models::CreateFeedPostParams | Да |  |
| broadcast_id | String | Нет |  |
| sso | String | Нет |  |

## Ответ

Возвращает: [`CreateFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_public_200_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример create_feed_post_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let create_feed_post_params: models::CreateFeedPostParams = models::CreateFeedPostParams {
    title: "Acme Product Launch".to_string(),
    body: "We're excited to announce our new product line that will change the way teams collaborate.".to_string(),
    slug: "news/product-launch".to_string(),
    links: Some(vec![models::FeedPostLink { url: "https://acme.com/launch".to_string(), title: Some("Launch details".to_string()) }]),
    media: Some(vec![models::FeedPostMediaItem { assets: Some(vec![models::FeedPostMediaItemAsset { url: "https://cdn.acme.com/images/launch.jpg".to_string(), mime_type: Some("image/jpeg".to_string()) }]), caption: Some("Hero image".to_string()) }]),
    tags: Some(vec!["announcement".to_string(), "product".to_string()]),
};

let params: CreateFeedPostPublicParams = CreateFeedPostPublicParams {
    tenant_id: "acme-corp-tenant".to_string(),
    create_feed_post_params,
    broadcast_id: Some("broadcast-2026-03-25".to_string()),
    sso: Some("sso-token-xyz789".to_string()),
};

let response: CreateFeedPostPublic200Response = create_feed_post_public(&configuration, params).await?;
[inline-code-end]