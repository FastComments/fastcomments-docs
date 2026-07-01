## Parameters

| Ad | Tip | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| year_number | f64 | Hayır |  |
| month_number | f64 | Hayır |  |
| day_number | f64 | Hayır |  |
| skip | f64 | Hayır |  |

## Response

Döndürür: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_daily_usages_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_tenant_daily_usages Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_daily_usage(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetTenantDailyUsagesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        year_number: Some(2023.0),
        month_number: Some(7.0),
        day_number: Some(15.0),
        skip: Some(0.0),
    };
    let _response: GetTenantDailyUsagesResponse = get_tenant_daily_usages(configuration, params).await?;
    Ok(())
}
[inline-code-end]