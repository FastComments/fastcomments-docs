## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| year_number | f64 | לא |  |
| month_number | f64 | לא |  |
| day_number | f64 | לא |  |
| skip | f64 | לא |  |

## תגובה

מחזיר: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_daily_usages_200_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_tenant_daily_usages'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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