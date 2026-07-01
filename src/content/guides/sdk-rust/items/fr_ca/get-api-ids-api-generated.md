## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| text_search | String | Non |  |
| by_ip_from_comment | String | Non |  |
| filters | String | Non |  |
| search_filters | String | Non |  |
| after_id | String | Non |  |
| demo | bool | Non |  |
| sso | String | Non |  |

## Réponse

Renvoie : [`ModerationApiGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_get_comment_ids_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple get_api_ids'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetApiIdsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        text_search: Some("breaking news".to_string()),
        by_ip_from_comment: None,
        filters: Some("status:approved".to_string()),
        search_filters: None,
        after_id: None,
        demo: Some(false),
        sso: Some("sso-token".to_string()),
    };
    let _response = get_api_ids(&config, params).await?;
    Ok(())
}
[inline-code-end]