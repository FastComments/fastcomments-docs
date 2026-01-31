## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| year_number | f64 | No |  |
| month_number | f64 | No |  |
| day_number | f64 | No |  |
| skip | f64 | No |  |

## Response

Returns: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_daily_usages_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_tenant_daily_usages Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params: GetTenantDailyUsagesParams = GetTenantDailyUsagesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        year_number: Some(2025.0),
        month_number: Some(8.0),
        day_number: Some(15.0),
        skip: Some(0.0),
    };
    let daily_usages: GetTenantDailyUsages200Response =
        get_tenant_daily_usages(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
