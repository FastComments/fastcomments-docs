## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| year_number | f64 | 否 |  |
| month_number | f64 | 否 |  |
| day_number | f64 | 否 |  |
| skip | f64 | 否 |  |

## 响应

返回: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_daily_usages_200_response.rs)

## 示例

[inline-code-attrs-start title = 'get_tenant_daily_usages 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---