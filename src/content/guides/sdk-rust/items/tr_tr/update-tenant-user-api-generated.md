## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| id | String | Evet |  |
| update_tenant_user_body | models::UpdateTenantUserBody | Evet |  |
| update_comments | String | Hayır |  |

## Yanıt

Döndürür: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Örnek

[inline-code-attrs-start title = 'update_tenant_user Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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