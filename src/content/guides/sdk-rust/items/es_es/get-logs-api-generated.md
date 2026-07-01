## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| comment_id | String | Sí |  |
| sso | String | No |  |

## Respuesta

Devuelve: [`ModerationApiGetLogsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_get_logs_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo get_logs'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetLogsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article-12345".to_string(),
        sso: Some("user@example.com".to_string()),
    };
    let response = get_logs(&configuration, params).await?;
    let _ = response;
    Ok(())
}
[inline-code-end]

---