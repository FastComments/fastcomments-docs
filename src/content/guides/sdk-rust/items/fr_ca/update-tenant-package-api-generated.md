## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| id | String | Oui |  |
| update_tenant_package_body | models::UpdateTenantPackageBody | Oui |  |

## Réponse

Retourne : [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de update_tenant_package'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_update_tenant_package() -> Result<FlagCommentPublic200Response, Error> {
    let params: UpdateTenantPackageParams = UpdateTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "pro-plan-2026".to_string(),
        update_tenant_package_body: models::UpdateTenantPackageBody {
            name: Some("Pro Plan".to_string()),
            description: Some("Priority support, custom branding, and advanced moderation tools".to_string()),
            enabled: Some(true),
            monthly_price_cents: Some(1999),
            features: Some(vec![
                "priority_support".to_string(),
                "custom_branding".to_string(),
                "advanced_moderation".to_string(),
            ]),
        },
    };

    let response: FlagCommentPublic200Response = update_tenant_package(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---