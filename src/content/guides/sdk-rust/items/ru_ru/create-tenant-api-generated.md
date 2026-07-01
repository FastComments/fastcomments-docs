## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_tenant_body | models::CreateTenantBody | Yes |  |

## Ответ

Возвращает: [`CreateTenantResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_tenant_response.rs)

## Пример

[inline-code-attrs-start title = 'create_tenant Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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