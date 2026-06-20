## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| user_id | String | Nein |  |
| trust_factor | String | Nein |  |
| sso | String | Nein |  |

## Antwort

Gibt zurück: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_user_trust_factor_response.rs)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für set_trust_factor'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn update_user_trust() -> Result<SetUserTrustFactorResponse, Error> {
    let params: SetTrustFactorParams = SetTrustFactorParams {
        user_id: Some("user-9821".to_string()),
        trust_factor: Some("high".to_string()),
        sso: Some("okta-acme-corp".to_string()),
    };

    let response: SetUserTrustFactorResponse = set_trust_factor(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---