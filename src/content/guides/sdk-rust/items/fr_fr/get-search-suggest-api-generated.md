## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenant_id | String | Oui |  |
| text_search | String | Non |  |
| sso | String | Non |  |

## Réponse

Retourne : [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_suggest_response.rs)

## Exemple

[inline-code-attrs-start title = 'get_search_suggest Exemple'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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