## パラメータ

| Name | Type | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| create_tenant_body | models::CreateTenantBody | はい |  |

## レスポンス

戻り値: [`CreateTenant200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_tenant_200_response.rs)

## 例

[inline-code-attrs-start title = 'create_tenant の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: CreateTenantParams = CreateTenantParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_tenant_body: models::CreateTenantBody {
            name: "Acme Corporation".to_string(),
            primary_domain: Some("acme.example.com".to_string()),
            admin_email: Some("admin@acme.example.com".to_string()),
            api_domain_configuration: Some(models::ApiDomainConfiguration {
                primary_domain: Some("acme.example.com".to_string()),
                allowed_origins: Some(vec![
                    "https://acme.example.com".to_string(),
                    "https://www.acme.com".to_string()
                ]),
                ..Default::default()
            }),
            billing_info: Some(models::BillingInfo {
                plan: "business".to_string(),
                company_name: Some("Acme Corporation".to_string()),
                contact_email: Some("billing@acme.example.com".to_string()),
                ..Default::default()
            }),
            imported_sites: Some(vec![models::ImportedSiteType {
                site_id: "news/site-1".to_string(),
                origin: Some("https://news.acme.com".to_string()),
                ..Default::default()
            }]),
            ..Default::default()
        },
    };
    let response: CreateTenant200Response = create_tenant(&configuration, params).await?;
    Ok(())
}
[inline-code-end]