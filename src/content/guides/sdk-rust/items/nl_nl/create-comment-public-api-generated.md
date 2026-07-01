## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenant_id | String | Ja |  |
| url_id | String | Ja |  |
| broadcast_id | String | Ja |  |
| comment_data | models::CommentData | Ja |  |
| session_id | String | Nee |  |
| sso | String | Nee |  |

## Respons

Retourneert: [`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/save_comments_response_with_presence.rs)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld van create_comment_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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