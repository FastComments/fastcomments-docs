## Parámetros

| Nombre | Type | Requerido | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |

## Respuesta

Devuelve: [`GetPagesApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_pages_api_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_pages'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetPagesParams = GetPagesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        path: Some("news/article".to_string()),
        limit: Some(25),
        cursor: Some("cursor_01AZ".to_string()),
    };
    let pages: GetPagesApiResponse = get_pages(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---