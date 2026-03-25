## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| year_number | f64 | Όχι |  |
| month_number | f64 | Όχι |  |
| day_number | f64 | Όχι |  |
| skip | f64 | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_daily_usages_200_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_tenant_daily_usages'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_usage() -> Result<GetTenantDailyUsages200Response, Error> {
    let params = GetTenantDailyUsagesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        year_number: Some(2026.0),
        month_number: Some(3.0),
        day_number: Some(25.0),
        skip: Some(0.0),
    };
    let response = get_tenant_daily_usages(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---