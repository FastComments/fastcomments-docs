## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| year_number | f64 | 否 |  |
| month_number | f64 | 否 |  |
| day_number | f64 | 否 |  |
| skip | f64 | 否 |  |

## 响应

返回：[`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_daily_usages_response.rs)

## 示例

[inline-code-attrs-start title = 'get_tenant_daily_usages 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---