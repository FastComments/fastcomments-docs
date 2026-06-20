## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| url_id | String | Oui |  |
| broadcast_id | String | Oui |  |
| comment_data | models::CommentData | Oui |  |
| session_id | String | Non |  |
| sso | String | Non |  |

## Réponse

Retourne : [`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/save_comments_response_with_presence.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de create_comment_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn post_public_comment(configuration: &configuration::Configuration) -> Result<SaveCommentsResponseWithPresence, Error> {
    let params: CreateCommentPublicParams = CreateCommentPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/world/economic-update-2026".to_string(),
        broadcast_id: "broadcast-2026-06-19-001".to_string(),
        comment_data: models::CommentData {
            content: "Great analysis — this clarified a lot of the market dynamics.".to_string(),
            ..Default::default()
        },
        session_id: Some("sess-9f8e7d6c".to_string()),
        sso: Some("sso-jwt-eyJhbGciOi...".to_string()),
    };
    let response: SaveCommentsResponseWithPresence = create_comment_public(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---