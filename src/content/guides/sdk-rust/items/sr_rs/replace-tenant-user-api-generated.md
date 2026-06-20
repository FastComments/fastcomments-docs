## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |
| replace_tenant_user_body | models::ReplaceTenantUserBody | Да |  |
| update_comments | String | Не |  |

## Одговор

Враћа: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Пример

[inline-code-attrs-start title = 'replace_tenant_user Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: ReplaceTenantUserParams = ReplaceTenantUserParams {
    tenant_id: "acme-corp-tenant".to_string(),
    id: "user-123".to_string(),
    replace_tenant_user_body: models::ReplaceTenantUserBody {
        user_id: "user-123".to_string(),
        email: "jane.doe@acme.com".to_string(),
        display_name: "Jane Doe".to_string(),
        roles: vec!["editor".to_string()],
    },
    update_comments: Some("propagate".to_string()),
};

let response: ApiEmptyResponse = replace_tenant_user(&configuration, params).await?;
[inline-code-end]

---