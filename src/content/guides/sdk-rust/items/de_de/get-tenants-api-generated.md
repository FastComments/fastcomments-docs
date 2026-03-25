---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| meta | String | Nein |  |
| skip | f64 | Nein |  |

## Antwort

Gibt zurück: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenants_200_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_tenants Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetTenantsParams = GetTenantsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        meta: Some("news/article".to_string()),
        skip: Some(10.0),
    };
    let response: GetTenants200Response = get_tenants(&configuration, params).await?;
    println!("{:#?}", response);
    Ok(())
}
[inline-code-end]

---