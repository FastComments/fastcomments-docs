## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| sso | String | Ne |  |

## Odgovor

Vraća: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_manual_badges_response.rs)

## Primjer

[inline-code-attrs-start title = 'get_manual_badges Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_manual_badges() -> Result<(), Error> {
    let params: GetManualBadgesParams = GetManualBadgesParams {
        sso: Some(String::from("https://sso.acme-corp.com/authorize?tenant=acme-corp-tenant")),
    };
    let response: GetTenantManualBadgesResponse = get_manual_badges(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---