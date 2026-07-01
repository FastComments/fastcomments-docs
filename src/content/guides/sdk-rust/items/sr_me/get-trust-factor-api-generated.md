## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Yes |  |
| user_id | String | No |  |
| sso | String | No |  |

## Odgovor

Vraća: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_trust_factor_response.rs)

## Primjer

[inline-code-attrs-start title = 'get_trust_factor Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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