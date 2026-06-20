## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| meta | String | 否 |  |
| skip | f64 | 否 |  |

## 响应

返回：[`GetTenantsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenants_response.rs)

## 示例

[inline-code-attrs-start title = 'get_tenants 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_get_tenants() -> Result<(), Error> {
    let params: GetTenantsParams = GetTenantsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        meta: Some("include=domains,billing".to_string()),
        skip: Some(10.0),
    };
    let tenants: GetTenantsResponse = get_tenants(&configuration, params).await?;
    println!("{:#?}", tenants);
    Ok(())
}
[inline-code-end]

---