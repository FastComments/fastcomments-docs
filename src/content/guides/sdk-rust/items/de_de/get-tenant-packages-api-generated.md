## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenant_id | String | Ja |  |
| skip | f64 | Nein |  |

## Antwort

Rückgabe: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_packages_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_tenant_packages Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetTenantPackagesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(5.0),
    };
    let _resp = get_tenant_packages(&config, params).await?;
    Ok(())
}
[inline-code-end]

---