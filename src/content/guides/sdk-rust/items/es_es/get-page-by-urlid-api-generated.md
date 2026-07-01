## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| url_id | String | Sí |  |

## Respuesta

Devuelve: [`GetPageByUrlidApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_page_by_urlid_api_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo get_page_by_urlid'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetPageByUrlidParams {
        tenant_id: "acme-corp-tenant".into(),
        url_id: "news/article".into(),
    };
    let _response = get_page_by_urlid(&config, params).await?;
    Ok(())
}
[inline-code-end]

---