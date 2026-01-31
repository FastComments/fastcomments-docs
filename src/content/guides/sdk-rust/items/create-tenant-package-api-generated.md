## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_tenant_package_body | models::CreateTenantPackageBody | Yes |  |

## Response

Returns: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_tenant_package_200_response.rs)

## Example

[inline-code-attrs-start title = 'create_tenant_package Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_create_package() -> Result<(), Error> {
    let params: CreateTenantPackageParams = CreateTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_tenant_package_body: models::CreateTenantPackageBody {
            package_name: Some("Acme Enterprise".to_string()),
            plan_id: Some("enterprise-annual".to_string()),
            seats: Some(200),
            enable_auto_renew: Some(true),
            custom_config: Some(models::CustomConfigParameters {
                allowed_domains: Some(vec!["acme.com".to_string(), "partners.acme.com".to_string()]),
                ..Default::default()
            }),
            ..Default::default()
        },
    };
    let created: CreateTenantPackage200Response = create_tenant_package(&configuration, params).await?;
    println!("{:#?}", created);
    Ok(())
}
[inline-code-end]
