## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| user_id | String | Non |  |
| trust_factor | String | Non |  |
| sso | String | Non |  |

## Réponse

Retourne : [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_user_trust_factor_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple set_trust_factor'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn update_trust() -> Result<(), Error> {
    let params = SetTrustFactorParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-123".to_string()),
        trust_factor: Some("high".to_string()),
        sso: Some("sso-token-xyz".to_string()),
    };
    let _response = set_trust_factor(&configuration, params).await?;
    Ok(())
}
[inline-code-end]