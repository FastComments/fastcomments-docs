## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_tenant_body | models::CreateTenantBody | Yes |  |

## 응답

반환: [`CreateTenantResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_tenant_response.rs)

## 예시

[inline-code-attrs-start title = 'create_tenant 예시'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let create_body = models::CreateTenantBody {
        name: "Acme Corp".into(),
        domain: ApiDomainConfiguration {
            domain_name: "acme.example.com".into(),
            ..Default::default()
        },
        imported_site_type: Some(ImportedSiteType::NewsArticle),
        billing_info: Some(BillingInfo {
            plan: "enterprise".into(),
            ..Default::default()
        }),
        ..Default::default()
    };
    let params = CreateTenantParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_tenant_body: create_body,
    };
    let _response: CreateTenantResponse = create_tenant(configuration, params).await?;
    Ok(())
}
[inline-code-end]