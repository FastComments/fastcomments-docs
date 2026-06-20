## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| year_number | f64 | No |  |
| month_number | f64 | No |  |
| day_number | f64 | No |  |
| skip | f64 | No |  |

## Response

Returns: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_daily_usages_response.rs)

## Example

[inline-code-attrs-start title = 'get_tenant_daily_usages Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetTenantDailyUsagesParams = GetTenantDailyUsagesParams {
        tenant_id: String::from("acme-corp-tenant"),
        year_number: Some(2026.0),
        month_number: Some(6.0),
        day_number: Some(19.0),
        skip: Some(0.0),
    };
    let daily_usages: GetTenantDailyUsagesResponse =
        get_tenant_daily_usages(&configuration, params).await?;
    let _ = daily_usages;
    Ok(())
}
[inline-code-end]
