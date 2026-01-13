## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| meta | String | 否 |  |
| skip | f64 | 否 |  |

## 响应

返回：[`GetTenants200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenants_200_response.rs)

## 示例

[inline-code-attrs-start title = 'get_tenants 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetTenantsParams = GetTenantsParams {
        tenant_id: String::from("acme-corp-tenant"),
        meta: Some(String::from("include=domains,settings")),
        skip: Some(10.0),
    };
    let response: GetTenants200Response = get_tenants(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---