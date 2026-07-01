## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenant_id | String | Sí |  |
| value | String | No |  |
| sso | String | No |  |

## Respuesta

Devuelve: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_page_search_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_search_pages'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetSearchPagesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        value: Some("news/article".to_string()),
        sso: Some("sso-token-123".to_string()),
    };
    let response: ModerationPageSearchResponse = get_search_pages(&configuration, params).await?;
    Ok(())
}
[inline-code-end]