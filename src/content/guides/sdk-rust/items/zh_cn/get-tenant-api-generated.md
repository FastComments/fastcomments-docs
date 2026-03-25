## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |

## 响应

返回: [`GetTenant200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_200_response.rs)

## 示例

[inline-code-attrs-start title = 'get_tenant 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_get_tenant() -> Result<GetTenant200Response, Error> {
    let params: GetTenantParams = GetTenantParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/site-42".to_string(),
        expand: Some(vec!["domains".to_string(), "billing".to_string()]),
    };
    let tenant: GetTenant200Response = get_tenant(&configuration, params).await?;
    Ok(tenant)
}
[inline-code-end]

---