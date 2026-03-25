## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|-------------|-------------|
| tenant_id | String | Sì |  |
| id | String | Sì |  |

## Risposta

Restituisce: [`GetTenant200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_200_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_tenant'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_get_tenant() -> Result<GetTenant200Response, Error> {
    let params: GetTenantParams = GetTenantParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/site-42".to_string(),
        expand: Some(vec!["domains".to_string(), "billing".to_string()]),
    };
    let tenant: GetTenant200Response = get_tenant(&configuration, params).await?;
    Ok(tenant)
}
[inline-code-end]

---