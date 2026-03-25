## Parámetros

| Name | Type | Requerido | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| id | String | Sí |  |
| update_tenant_package_body | models::UpdateTenantPackageBody | Sí |  |

## Respuesta

Devuelve: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de update_tenant_package'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params: UpdateTenantPackageParams = UpdateTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "pkg-professional-2026".to_string(),
        update_tenant_package_body: models::UpdateTenantPackageBody {
            name: "Acme Professional".to_string(),
            plan: "professional".to_string(),
            enabled: Some(true),
            api_status: Some(ApiStatus::Enabled),
            custom_config: Some(CustomConfigParameters {
                moderation_webhook: Some("https://acme.example.com/hooks/moderation".to_string()),
                sso_security_level: Some(SsoSecurityLevel::Strict),
            }),
            tos_config: Some(TosConfig {
                enabled: Some(true),
                url: Some("https://acme.example.com/terms".to_string()),
            }),
        },
    };
    let response: FlagCommentPublic200Response = update_tenant_package(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---