## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenant_id | String | Oui |  |
| value | String | Non |  |
| filters | String | Non |  |
| search_filters | String | Non |  |
| sso | String | Non |  |

## Réponse

Renvoie : [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_comment_search_response.rs)

## Exemple

[inline-code-attrs-start title = 'get_search_comments_summary Exemple'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetSearchCommentsSummaryParams {
        tenant_id: "acme-corp-tenant".to_string(),
        value: Some("news/article".to_string()),
        filters: Some("status:approved".to_string()),
        search_filters: Some("author:john".to_string()),
        sso: Some("sso-token-123".to_string()),
    };
    let _response = get_search_comments_summary(configuration, params).await?;
    Ok(())
}
[inline-code-end]