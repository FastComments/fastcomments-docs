## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| skip | f64 | 否 |  |

## 响应

返回: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_packages_response.rs)

## 示例

[inline-code-attrs-start title = 'get_tenant_packages 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetTenantPackagesParams = GetTenantPackagesParams {
        tenant_id: String::from("acme-corp-tenant"),
        skip: Some(10.0),
    };
    let response: GetTenantPackagesResponse = get_tenant_packages(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---