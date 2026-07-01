---
## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| id | String | Sì |  |
| update_tenant_package_body | models::UpdateTenantPackageBody | Sì |  |

## Risposta

Restituisce: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di update_tenant_package'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_update(configuration: &configuration::Configuration) -> Result<(), Error> {
    let body = models::UpdateTenantPackageBody {
        plan: Some("enterprise".to_string()),
        renewal_date: Some("2024-12-31".to_string()),
        ..Default::default()
    };
    let params = UpdateTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "pkg-2024".to_string(),
        update_tenant_package_body: body,
    };
    let _: ApiEmptyResponse = update_tenant_package(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---