## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_tenant_body | models::CreateTenantBody | Yes |  |

## Response

Returns: [`CreateTenant200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_tenant_200_response.rs)

## Example

[inline-code-attrs-start title = 'create_tenant Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: CreateTenantParams = CreateTenantParams {
    tenant_id: "acme-corp-tenant".to_string(),
    create_tenant_body: models::CreateTenantBody {
        name: "Acme Corp".to_string(),
        domain: "acme.com".to_string(),
        contact_email: Some("admin@acme.com".to_string()),
        billing_info: Some(models::BillingInfo {
            plan: "enterprise".to_string(),
            billing_contact: Some("billing@acme.com".to_string()),
        }),
        api_domain_configuration: Some(models::ApiDomainConfiguration {
            allowed_origins: Some(vec![
                "https://acme.com".to_string(),
                "https://www.acme.com".to_string()
            ]),
            api_key_required: Some(true),
        }),
        imported_site_types: Some(vec![models::ImportedSiteType {
            site_type: "news/article".to_string(),
            enabled: Some(true),
        }]),
    },
};
let response: CreateTenant200Response = create_tenant(&configuration, params).await?;
[inline-code-end]
