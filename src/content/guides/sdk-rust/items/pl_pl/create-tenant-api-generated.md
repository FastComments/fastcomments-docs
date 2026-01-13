---
## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| create_tenant_body | models::CreateTenantBody | Tak |  |

## Odpowiedź

Zwraca: [`CreateTenant200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_tenant_200_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład create_tenant'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: CreateTenantParams = CreateTenantParams {
        tenant_id: "acme-news-tenant".to_string(),
        create_tenant_body: models::CreateTenantBody {
            name: "Acme News".to_string(),
            domain: Some("news.acme.com".to_string()),
            api_domain_configuration: Some(models::ApiDomainConfiguration {
                domain: "api.news.acme.com".to_string(),
                enforce_https: true,
            }),
            billing_info: Some(models::BillingInfo {
                contact_email: "billing@acme.com".to_string(),
                plan_id: "pro_monthly".to_string(),
            }),
            imported_site_type: Some(models::ImportedSiteType::Articles),
            ..Default::default()
        },
    };

    let created: CreateTenant200Response = create_tenant(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---