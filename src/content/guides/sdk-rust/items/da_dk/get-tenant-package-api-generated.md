## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |

## Svar

Returnerer: [`GetTenantPackage200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_package_200_response.rs)

## Eksempel

[inline-code-attrs-start title = 'get_tenant_package Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_tenant_package() -> Result<GetTenantPackage200Response, Error> {
    let params: GetTenantPackageParams = GetTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "package-basic-2026".to_string(),
    };
    let include_metadata: Option<bool> = Some(true);
    let package: GetTenantPackage200Response = get_tenant_package(&configuration, params).await?;
    Ok(package)
}
[inline-code-end]

---