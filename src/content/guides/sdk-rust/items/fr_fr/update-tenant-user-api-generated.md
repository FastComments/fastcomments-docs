## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|------------|-------------|
| tenant_id | String | Oui |  |
| id | String | Oui |  |
| update_tenant_user_body | models::UpdateTenantUserBody | Oui |  |
| update_comments | String | Non |  |

## Réponse

Renvoie : [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de update_tenant_user'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_update_tenant_user(configuration: &configuration::Configuration) -> Result<FlagCommentPublic200Response, Error> {
    let params: UpdateTenantUserParams = UpdateTenantUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-7b9f".to_string(),
        update_tenant_user_body: models::UpdateTenantUserBody {
            email: Some("jane.doe@acme.com".to_string()),
            display_name: Some("Jane Doe".to_string()),
            username: Some("jdoe".to_string()),
            role: Some("moderator".to_string()),
        },
        update_comments: Some("Promoted to moderator to handle flagged comments".to_string()),
    };
    let response = update_tenant_user(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---