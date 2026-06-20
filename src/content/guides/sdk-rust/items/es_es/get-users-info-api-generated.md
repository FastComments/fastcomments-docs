Información masiva de usuarios para un tenant. Dado userIds, devuelve la información de visualización de User / SSOUser.
Usado por el widget de comentarios para enriquecer a los usuarios que acaban de aparecer mediante un evento de presencia.
Sin contexto de página: la privacidad se aplica de forma uniforme (los perfiles privados se ocultan).

## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| ids | String | Sí |  |

## Respuesta

Devuelve: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_users_info'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetUsersInfoParams = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "alice@example.com,bob@example.com,carol@example.com".to_string(),
    page_size: Some(100),
};
let users_response: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]

---