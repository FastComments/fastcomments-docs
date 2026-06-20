## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |

## 响应

返回: [`GetTenantResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_response.rs)

## 示例

[inline-code-attrs-start title = 'get_tenant 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_tenant() -> Result<(), Error> {
    let params: GetTenantParams = GetTenantParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article".to_string(),
    };
    let include_subdomains: Option<bool> = Some(true);
    let tenant: GetTenantResponse = get_tenant(&configuration, params).await?;
    println!("{:#?}", tenant);
    Ok(())
}
[inline-code-end]

---