## Parámetros

| Name | Type | Requerido | Description |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| url_id | String | Sí |  |
| title | String | No |  |

## Respuesta

Devuelve: `CreateV1PageReact`

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de create_v1_page_react'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<CreateV1PageReact, Error> {
    let params = CreateV1PageReactParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article/2026/06/launch".to_string(),
        title: Some("Acme Launch Coverage".to_string()),
    };
    let reaction: CreateV1PageReact = create_v1_page_react(&configuration, params).await?;
    Ok(reaction)
}
[inline-code-end]

---