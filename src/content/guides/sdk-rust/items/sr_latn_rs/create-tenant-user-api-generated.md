## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| create_tenant_user_body | models::CreateTenantUserBody | Da |  |

## Odgovor

Vraća: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_tenant_user_200_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer za create_tenant_user'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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