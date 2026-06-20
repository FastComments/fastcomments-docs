---
Comentaristas anteriores en la página que NO están actualmente en línea. Ordenados por displayName.
Usar esto después de agotar /users/online para mostrar una sección "Members".
Paginación por cursor en commenterName: el servidor avanza por el índice parcial {tenantId, urlId, commenterName} desde afterName hacia adelante mediante $gt, sin coste de $skip.

## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| url_id | String | Sí |  |
| after_name | String | No |  |
| after_user_id | String | No |  |

## Respuesta

Devuelve: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_offline_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_offline_users() -> Result<PageUsersOfflineResponse, Error> {
    let params: GetOfflineUsersParams = GetOfflineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/world/today".to_string(),
        after_name: Some("jane.smith".to_string()),
        after_user_id: Some("user-1024".to_string()),
    };
    let response: PageUsersOfflineResponse = get_offline_users(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---