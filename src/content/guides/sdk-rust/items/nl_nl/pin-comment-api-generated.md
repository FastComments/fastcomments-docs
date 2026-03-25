## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenant_id | String | Ja |  |
| comment_id | String | Ja |  |
| broadcast_id | String | Ja |  |
| sso | String | Nee |  |

## Respons

Retourneert: [`PinComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/pin_comment_200_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'pin_comment Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: PinCommentParams = PinCommentParams {
    tenant_id: "acme-corp-tenant".to_string(),
    comment_id: "cmt-987654321".to_string(),
    broadcast_id: "news/article/2026-03-25".to_string(),
    sso: Some("user-12345-ssotoken".to_string()),
};
let response: PinComment200Response = pin_comment(&configuration, params).await?;
[inline-code-end]

---