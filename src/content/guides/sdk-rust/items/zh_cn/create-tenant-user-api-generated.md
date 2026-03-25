## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|-------------|
| tenant_id | String | 是 |  |
| create_tenant_user_body | models::CreateTenantUserBody | 是 |  |

## 响应

返回: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_tenant_user_200_response.rs)

## 示例

[inline-code-attrs-start title = 'create_tenant_user 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: CreateTenantUserParams = CreateTenantUserParams {
    tenant_id: "acme-corp-tenant".to_string(),
    create_tenant_user_body: models::CreateTenantUserBody {
        username: "jane.doe".to_string(),
        email: "jane.doe@acme.com".to_string(),
        display_name: Some("Jane Doe".to_string()),
        roles: Some(vec!["reader".to_string(), "commenter".to_string()]),
        locale: Some("en-US".to_string()),
        is_verified: Some(true),
    },
};

let response: CreateTenantUser200Response = create_tenant_user(&configuration, params).await?;
[inline-code-end]

---