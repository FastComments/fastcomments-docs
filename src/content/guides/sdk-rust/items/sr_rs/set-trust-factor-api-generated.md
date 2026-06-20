## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| user_id | String | Не |  |
| trust_factor | String | Не |  |
| sso | String | Не |  |

## Одговор

Враћа: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_user_trust_factor_response.rs)

## Пример

[inline-code-attrs-start title = 'set_trust_factor Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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