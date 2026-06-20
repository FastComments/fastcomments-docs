## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|--------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |

## Respons

Retourneert: [`GetTenantResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_tenant Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_tenant() -> Result<(), Error> {
    let params: GetTenantParams = GetTenantParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article".to_string(),
    };
    let include_subdomains: Option<bool> = Some(true);
    let tenant: GetTenantResponse = get_tenant(&configuration, params).await?;
    println!("{:#?}", tenant);
    Ok(())
}
[inline-code-end]

---