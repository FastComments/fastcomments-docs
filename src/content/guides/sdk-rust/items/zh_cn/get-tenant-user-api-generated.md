## 参数

| Name | 类型 | 必需 | 描述 |
|------|------|------|------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |

## 响应

返回：[`GetTenantUser200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_user_200_response.rs)

## 示例

[inline-code-attrs-start title = 'get_tenant_user 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetTenantUserParams = GetTenantUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-9a4f2e".to_string(),
        expand: Some(vec!["roles".to_string(), "preferences".to_string()]),
    };
    let user_response: GetTenantUser200Response = get_tenant_user(&configuration, params).await?;
    println!("{:#?}", user_response);
    Ok(())
}
[inline-code-end]

---