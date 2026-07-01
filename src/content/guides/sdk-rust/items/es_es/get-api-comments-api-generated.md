## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|--------------|-------------|
| tenant_id | String | Yes |  |
| page | f64 | No |  |
| count | f64 | No |  |
| text_search | String | No |  |
| by_ip_from_comment | String | No |  |
| filters | String | No |  |
| search_filters | String | No |  |
| sorts | String | No |  |
| demo | bool | No |  |
| sso | String | No |  |

## Respuesta

Devuelve: [`ModerationApiGetCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_get_comments_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo get_api_comments'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetApiCommentsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        page: Some(1.0),
        count: Some(20.0),
        text_search: Some("rust".to_string()),
        by_ip_from_comment: None,
        filters: Some("status:approved".to_string()),
        search_filters: Some("author:john".to_string()),
        sorts: Some("date:desc".to_string()),
        demo: Some(false),
        sso: None,
    };
    let _response = get_api_comments(&configuration, params).await?;
    Ok(())
}
[inline-code-end]