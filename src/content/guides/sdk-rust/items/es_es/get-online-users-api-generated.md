Actualmente, los espectadores en línea de una página: personas cuya sesión de websocket está suscrita a la página en este momento. Devuelve **anonCount + totalCount** (suscriptores a nivel de sala, incluidos los espectadores anónimos que no enumeramos).

## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenant_id | String | Sí |  |
| url_id | String | Sí |  |
| after_name | String | No |  |
| after_user_id | String | No |  |

## Respuesta

Devuelve: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_online_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_online_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetOnlineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        after_name: Some("john_doe".to_string()),
        after_user_id: Some("user-123".to_string()),
    };
    let _response: PageUsersOnlineResponse = get_online_users(&config, params).await?;
    Ok(())
}
[inline-code-end]