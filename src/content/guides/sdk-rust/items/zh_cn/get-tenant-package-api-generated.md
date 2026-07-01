## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## 响应

返回: [`GetTenantPackageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_package_response.rs)

## 示例

[inline-code-attrs-start title = 'get_tenant_package 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article".to_string(),
    };
    let _response: GetTenantPackageResponse = get_tenant_package(configuration, params).await?;
    Ok(())
}
[inline-code-end]