## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| id | String | Sì |  |
| update_tenant_body | models::UpdateTenantBody | Sì |  |

## Risposta

Restituisce: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di update_tenant'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: UpdateTenantParams = UpdateTenantParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "site-1234".to_string(),
        update_tenant_body: models::UpdateTenantBody {
            name: Some("Acme Corp Comments".to_string()),
            admin_email: Some("admin@acme.com".to_string()),
            is_active: Some(true),
            billing_info: Some(models::BillingInfo {
                plan: "professional".to_string(),
                contact_email: "billing@acme.com".to_string(),
            }),
            domain_configuration: Some(models::ApiDomainConfiguration {
                primary_domain: "comments.acme.com".to_string(),
            }),
        },
    };

    let response: ApiEmptyResponse = update_tenant(configuration, params).await?;
    let _ = response;
    Ok(())
}
[inline-code-end]