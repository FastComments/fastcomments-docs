## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| id | String | Sì |  |

## Risposta

Restituisce: [`GetTenantPackageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_package_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_tenant_package'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_get_tenant_package() -> Result<GetTenantPackageResponse, Error> {
    let params: GetTenantPackageParams = GetTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "pkg-premium-001".to_string(),
        include_related: Some(true),
    };
    let response: GetTenantPackageResponse = get_tenant_package(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---