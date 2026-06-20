## Параметри

| Назва | Тип | Обов'язкове | Опис |
|------|------|----------|-------------|
| user_id | String | Ні |  |
| trust_factor | String | Ні |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_user_trust_factor_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад set_trust_factor'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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