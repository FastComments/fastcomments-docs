## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| update_tenant_user_body | models::UpdateTenantUserBody | 是 |  |
| update_comments | String | 否 |  |

## 响应

返回: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 示例

[inline-code-attrs-start title = 'update_tenant_user 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = UpdateTenantUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-9876".to_string(),
        update_tenant_user_body: models::UpdateTenantUserBody {
            email: "jane.doe@example.com".to_string(),
            role: "editor".to_string(),
        },
        update_comments: Some("Promoted to editor".to_string()),
    };
    let _ = update_tenant_user(configuration, params).await?;
    Ok(())
}
[inline-code-end]