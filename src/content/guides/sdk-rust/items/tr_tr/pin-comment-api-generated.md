## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| comment_id | String | Evet |  |
| broadcast_id | String | Evet |  |
| sso | String | Hayır |  |

## Yanıt

Döndürür: [`PinComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/pin_comment_200_response.rs)

## Örnek

[inline-code-attrs-start title = 'pin_comment Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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