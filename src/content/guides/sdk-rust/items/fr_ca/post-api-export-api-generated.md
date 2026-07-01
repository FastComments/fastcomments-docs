## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| text_search | String | Non |  |
| by_ip_from_comment | String | Non |  |
| filters | String | Non |  |
| search_filters | String | Non |  |
| sorts | String | Non |  |
| sso | String | Non |  |

## Réponse

Renvoie : [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_export_response.rs)

## Exemple

[inline-code-attrs-start title = 'post_api_export Exemple'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn export_moderation() -> Result<(), Error> {
    let params = PostApiExportParams {
        tenant_id: "acme-corp-tenant".to_string(),
        text_search: Some("news/article".to_string()),
        by_ip_from_comment: Some("203.0.113.42".to_string()),
        filters: Some("status:pending".to_string()),
        search_filters: Some("created_at>2023-01-01".to_string()),
        sorts: Some("created_at_desc".to_string()),
        sso: None,
    };
    let _response = post_api_export(&configuration, params).await?;
    Ok(())
}
[inline-code-end]