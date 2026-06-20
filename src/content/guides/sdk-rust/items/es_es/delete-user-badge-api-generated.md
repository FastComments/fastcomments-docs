## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| id | String | Sí |  |

## Respuesta

Devuelve: [`ApiEmptySuccessResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_success_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de delete_user_badge'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: DeleteUserBadgeParams = DeleteUserBadgeParams {
    tenant_id: "acme-newsroom-tenant".to_string(),
    id: "badge-moderator-001".to_string(),
};
let include_related: Option<bool> = Some(false);
let result: ApiEmptySuccessResponse = delete_user_badge(&configuration, params).await?;
[inline-code-end]

---