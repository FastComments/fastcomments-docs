## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| comment_id | String | Oui |  |
| broadcast_id | String | Oui |  |
| sso | String | Non |  |

## Réponse

Renvoie : [`PinComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/pin_comment_200_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de un_pin_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_unpin() -> Result<(), Error> {
    let params: UnPinCommentParams = UnPinCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-20260325-7a9".to_string(),
        broadcast_id: "news/homepage/launch-article".to_string(),
        sso: Some("sso-jwt-user-0a1b2c3d".to_string()),
    };
    let response: PinComment200Response = un_pin_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---