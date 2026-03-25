## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| year_number | f64 | No |  |
| month_number | f64 | No |  |
| day_number | f64 | No |  |
| skip | f64 | No |  |

## 响应

返回: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_daily_usages_200_response.rs)

## 示例

[inline-code-attrs-start title = 'get_tenant_daily_usages 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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