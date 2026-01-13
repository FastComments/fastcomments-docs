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
let create_tenant_user_body: models::CreateTenantUserBody = models::CreateTenantUserBody {
    email: "jane.doe@acme.com".to_string(),
    display_name: Some("Jane Doe".to_string()),
    role: Some("moderator".to_string()),
    external_id: Some("acme-12345".to_string()),
    subscribed_to_digest: Some(false),
};
let params: CreateTenantUserParams = CreateTenantUserParams {
    tenant_id: "acme-corp-tenant".to_string(),
    create_tenant_user_body,
};
let response: CreateTenantUser200Response = create_tenant_user(&configuration, params).await?;
[inline-code-end]

---