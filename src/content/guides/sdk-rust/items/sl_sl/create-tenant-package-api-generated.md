## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| create_tenant_package_body | models::CreateTenantPackageBody | Da |  |

## Odgovor

Vrne: [`CreateTenantPackageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_tenant_package_response.rs)

## Primer

[inline-code-attrs-start title = 'create_tenant_package Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = CreateTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_tenant_package_body: models::CreateTenantPackageBody {
            package_name: "Standard".to_string(),
            package_type: "news/article".to_string(),
            description: Some("Package for news articles".to_string()),
            ..Default::default()
        },
    };
    let _response = create_tenant_package(&configuration, params).await?;
    Ok(())
}
[inline-code-end]