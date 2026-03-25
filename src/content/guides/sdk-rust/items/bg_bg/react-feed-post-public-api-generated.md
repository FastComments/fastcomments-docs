## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| post_id | String | Да |  |
| react_body_params | models::ReactBodyParams | Да |  |
| is_undo | bool | Не |  |
| broadcast_id | String | Не |  |
| sso | String | Не |  |

## Отговор

Връща: [`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/react_feed_post_public_200_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример за react_feed_post_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: ReactFeedPostPublicParams = ReactFeedPostPublicParams {
    tenant_id: "acme-corp-tenant".to_string(),
    post_id: "news/world/2026-election".to_string(),
    react_body_params: models::ReactBodyParams { reaction: "like".to_string() },
    is_undo: Some(false),
    broadcast_id: Some("broadcast-2026-03-25".to_string()),
    sso: Some("sso-token-6f4e2b".to_string()),
};

let response: ReactFeedPostPublic200Response = react_feed_post_public(&configuration, params).await?;
[inline-code-end]

---