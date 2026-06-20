## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| id | String | Sí |  |
| update_tenant_user_body | models::UpdateTenantUserBody | Sí |  |
| update_comments | String | No |  |

## Respuesta

Devuelve: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de update_tenant_user'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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