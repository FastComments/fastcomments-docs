## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| comment_id | String | Ja |  |
| broadcast_id | String | Ja |  |
| sso | String | Nein |  |

## Antwort

Gibt zurück: [`PinComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/pin_comment_200_response.rs)

## Beispiel

[inline-code-attrs-start title = 'pin_comment Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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