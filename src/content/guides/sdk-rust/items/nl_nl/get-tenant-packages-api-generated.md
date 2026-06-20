---
## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenant_id | String | Ja |  |
| skip | f64 | Nee |  |

## Antwoord

Retourneert: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_packages_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_tenant_packages Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetTenantPackagesParams = GetTenantPackagesParams {
        tenant_id: String::from("acme-corp-tenant"),
        skip: Some(10.0),
    };
    let response: GetTenantPackagesResponse = get_tenant_packages(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---