## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|-------------|-------------|
| tenant_id | String | Sí |  |
| text_search | String | No |  |
| sso | String | No |  |

## Respuesta

Devuelve: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_suggest_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'get_search_suggest Ejemplo'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetSearchSuggestParams {
        tenant_id: "acme-corp-tenant".to_string(),
        text_search: Some("news/article".to_string()),
        sso: Some("sso-token-123".to_string()),
    };
    let _response: ModerationSuggestResponse = get_search_suggest(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---