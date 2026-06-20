## Paramètres

| Name | Type | Obligatoire | Description |
|------|------|------------|-------------|
| tenant_id | String | Oui |  |
| id | String | Oui |  |

## Réponse

Renvoie : [`ApiEmptySuccessResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_success_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de delete_user_badge'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: DeleteUserBadgeParams = DeleteUserBadgeParams {
    tenant_id: "acme-newsroom-tenant".to_string(),
    id: "badge-moderator-001".to_string(),
};
let include_related: Option<bool> = Some(false);
let result: ApiEmptySuccessResponse = delete_user_badge(&configuration, params).await?;
[inline-code-end]

---