## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| id | String | Da |  |
| update_tenant_user_body | models::UpdateTenantUserBody | Da |  |
| update_comments | String | Ne |  |

## Odgovor

Vraća: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Primjer

[inline-code-attrs-start title = 'Primjer update_tenant_user'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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