Información masiva de usuarios para un arrendatario. Dado userIds, devuelve información de visualización de User / SSOUser.  
Utilizado por el widget de comentarios para enriquecer a los usuarios que acaban de aparecer mediante un evento de presencia.  
Sin contexto de página: la privacidad se aplica uniformemente (los perfiles privados se ocultan).

## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|-------------|-------------|
| tenant_id | String | Sí |  |
| ids | String | Sí |  |

## Respuesta

Devuelve: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo get_users_info'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "user-1,user-2".to_string(),
};
let page: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]