## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |
| update_tenant_user_body | models::UpdateTenantUserBody | 예 |  |
| update_comments | String | 아니오 |  |

## 응답

반환: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 예제

[inline-code-attrs-start title = 'update_tenant_user 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: UpdateTenantUserParams = UpdateTenantUserParams {
    tenant_id: String::from("acme-corp-tenant"),
    id: String::from("user_42"),
    update_tenant_user_body: models::UpdateTenantUserBody {
        email: Some(String::from("alice.johnson@acme.com")),
        display_name: Some(String::from("Alice Johnson")),
        roles: Some(vec![String::from("editor")]),
        active: Some(true),
    },
    update_comments: Some(String::from("synchronize-profile-and-comments")),
};
let response: ApiEmptyResponse = update_tenant_user(&configuration, params).await?;
[inline-code-end]

---