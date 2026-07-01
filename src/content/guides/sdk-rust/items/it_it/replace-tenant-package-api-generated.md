## Parametri

| Nome | Tipo | Richiesto | Descrizione |
|------|------|----------|-------------|
| tenant_id | String | Sì |  |
| id | String | Sì |  |
| replace_tenant_package_body | models::ReplaceTenantPackageBody | Sì |  |

## Risposta

Restituisce: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio replace_tenant_package'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = ReplaceTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article".to_string(),
        replace_tenant_package_body: models::ReplaceTenantPackageBody {
            package_id: "premium-plan".to_string(),
            enabled: true,
            description: Some("Premium package for high traffic".to_string()),
        },
    };
    replace_tenant_package(&configuration, params).await?;
    Ok(())
}
[inline-code-end]