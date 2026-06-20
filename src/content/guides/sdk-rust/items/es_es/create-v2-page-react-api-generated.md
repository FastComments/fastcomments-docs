## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| url_id | String | Sí |  |
| id | String | Sí |  |
| title | String | No |  |

## Respuesta

Devuelve: `CreateV1PageReact`

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de create_v2_page_react'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_create_react() -> Result<CreateV1PageReact, Error> {
    let params: CreateV2PageReactParams = CreateV2PageReactParams {
        tenant_id: String::from("acme-corp-tenant"),
        url_id: String::from("news/2026/product-launch"),
        id: String::from("react-like"),
        title: Some(String::from("Product Launch Coverage")),
    };
    let response: CreateV1PageReact = create_v2_page_react(&config, params).await?;
    Ok(response)
}
[inline-code-end]

---