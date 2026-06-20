## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| text_search | String | No |  |
| sso | String | No |  |

## Respuesta

Devuelve: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_suggest_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_search_suggest'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_suggest() -> Result<(), Error> {
    let params: GetSearchSuggestParams = GetSearchSuggestParams {
        text_search: Some("news/article: presidential debate highlights".to_string()),
        sso: Some("acme-corp-tenant".to_string()),
    };
    let suggestion: ModerationSuggestResponse = get_search_suggest(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---