## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| meta | String | Nein |  |
| skip | f64 | Nein |  |

## Antwort

Gibt zurück: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenants_response.rs)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für get_tenants'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_get_tenants() -> Result<(), Error> {
    let params: GetTenantsParams = GetTenantsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        meta: Some("include=domains,billing".to_string()),
        skip: Some(10.0),
    };
    let tenants: GetTenantsResponse = get_tenants(&configuration, params).await?;
    println!("{:#?}", tenants);
    Ok(())
}
[inline-code-end]

---