## Paramètres

| Nom | Type | Obligatoire | Description |
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

[inline-code-attrs-start title = 'create_comment_public Exemple'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = CreateCommentPublicParams {
    tenant_id: "acme-corp-tenant".to_string(),
    url_id: "news/article-123".to_string(),
    broadcast_id: "broadcast-2023-09-01".to_string(),
    comment_data: models::CommentData {
        text: "Great read!".to_string(),
    },
    session_id: Some("session-abc123".to_string()),
    sso: Some("sso-token-xyz".to_string()),
};
let response = create_comment_public(&configuration, params).await?;
[inline-code-end]