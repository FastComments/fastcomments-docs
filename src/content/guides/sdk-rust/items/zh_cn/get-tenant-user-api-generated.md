## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |

## 响应

返回：[`GetTenantUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_user_response.rs)

## 示例

[inline-code-attrs-start title = 'get_tenant_user 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_tenant_user() -> Result<GetTenantUserResponse, Error> {
    let params: GetTenantUserParams = GetTenantUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-7b9a2".to_string(),
        include_profile: Some(true),
    };
    let response: GetTenantUserResponse = get_tenant_user(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---