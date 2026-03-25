---
## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| id | String | Sí |  |
| update_tenant_body | models::UpdateTenantBody | Sí |  |

## Respuesta

Devuelve: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de update_tenant'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: UpdateTenantParams = UpdateTenantParams {
    tenant_id: "acme-corp-tenant".to_string(),
    id: "site-42".to_string(),
    update_tenant_body: models::UpdateTenantBody {
        name: Some("Acme Corporation".to_string()),
        api_domain_configuration: Some(models::ApiDomainConfiguration {
            primary_domain: Some("comments.acme.com".to_string()),
            allowed_domains: Some(vec!["acme.com".to_string(), "www.acme.com".to_string()]),
        }),
        billing_info: Some(models::BillingInfo {
            plan: Some("business".to_string()),
            billing_contact_email: Some("billing@acme.com".to_string()),
        }),
        sso_security_level: Some(models::SsoSecurityLevel::Strict),
        custom_config_parameters: Some(models::CustomConfigParameters {
            max_comment_length: Some(2000),
            enable_moderation_queue: Some(true),
        }),
    },
};
let response: FlagCommentPublic200Response = update_tenant(&configuration, params).await?;
[inline-code-end]

---