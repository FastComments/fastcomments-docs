## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|------------|-------------|
| tenant_id | String | Oui |  |
| comment_id | String | Oui |  |
| include_email | bool | Non |  |
| include_ip | bool | Non |  |
| sso | String | Non |  |

## Réponse

Renvoie : [`ModerationApiCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_comment_response.rs)

## Exemple

[inline-code-attrs-start title = 'get_moderation_comment Exemple'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetModerationCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article-6789".to_string(),
        include_email: Some(true),
        include_ip: Some(true),
        sso: Some("sso-user-42".to_string()),
    };
    let _response = get_moderation_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]