## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenant_id | String | Да |  |
| user_id | String | Не |  |
| sso | String | Не |  |

## Одговор

Враћа: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_trust_factor_response.rs)

## Пример

[inline-code-attrs-start title = 'get_trust_factor Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetTrustFactorParams {
        tenant_id: "acme-corp-tenant".into(),
        user_id: Some("user-12345".into()),
        sso: Some("sso-provider".into()),
    };
    let _response = get_trust_factor(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---