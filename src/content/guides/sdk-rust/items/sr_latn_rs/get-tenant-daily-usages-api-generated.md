## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| year_number | f64 | Ne |  |
| month_number | f64 | Ne |  |
| day_number | f64 | Ne |  |
| skip | f64 | Ne |  |

## Odgovor

Vraća: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_daily_usages_200_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer get_tenant_daily_usages'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<GetTenantDailyUsages200Response, Error> {
    let params: GetTenantDailyUsagesParams = GetTenantDailyUsagesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        year_number: Some(2024.0),
        month_number: Some(9.0),
        day_number: Some(15.0),
        skip: Some(0.0),
    };
    let response: GetTenantDailyUsages200Response = get_tenant_daily_usages(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]