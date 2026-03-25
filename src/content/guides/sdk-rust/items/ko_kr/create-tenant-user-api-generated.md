## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| create_tenant_user_body | models::CreateTenantUserBody | 예 |  |

## 응답

반환: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_tenant_user_200_response.rs)

## 예제

[inline-code-attrs-start title = 'create_tenant_user 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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