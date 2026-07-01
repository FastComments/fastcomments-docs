## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Da |  |
| id | String | Da |  |
| update_tenant_body | models::UpdateTenantBody | Da |  |

## Odgovor

Vraća: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Primjer

[inline-code-attrs-start title = 'Primjer update_tenant'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = UpdateTenantParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "tenant-001".to_string(),
        update_tenant_body: UpdateTenantBody {
            description: Some("Primary tenant for Acme Corp".to_string()),
            ..Default::default()
        },
    };
    let _ = update_tenant(&configuration, params).await?;
    Ok(())
}
[inline-code-end]