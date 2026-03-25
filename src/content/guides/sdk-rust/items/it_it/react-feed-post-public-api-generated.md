## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Sì |  |
| post_id | String | Sì |  |
| react_body_params | models::ReactBodyParams | Sì |  |
| is_undo | bool | No |  |
| broadcast_id | String | No |  |
| sso | String | No |  |

## Risposta

Restituisce: [`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/react_feed_post_public_200_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di react_feed_post_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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