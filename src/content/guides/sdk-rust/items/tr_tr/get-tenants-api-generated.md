## Parametreler

| Ad | Tip | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| meta | String | Hayır |  |
| skip | f64 | Hayır |  |

## Yanıt

Döndürür: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenants_200_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_tenants Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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