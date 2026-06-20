## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| meta | String | Не |  |
| skip | f64 | Не |  |

## Одговор

Враћа: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenants_response.rs)

## Пример

[inline-code-attrs-start title = 'get_tenants Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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