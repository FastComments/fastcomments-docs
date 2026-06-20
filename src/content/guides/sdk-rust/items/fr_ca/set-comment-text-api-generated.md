## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| comment_id | String | Oui |  |
| broadcast_id | String | Oui |  |
| comment_text_update_request | models::CommentTextUpdateRequest | Oui |  |
| edit_key | String | Non |  |
| sso | String | Non |  |

## Réponse

Renvoie : [`PublicApiSetCommentTextResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/public_api_set_comment_text_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de set_comment_text'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn update_comment() -> Result<PublicApiSetCommentTextResponse, Error> {
    let params: SetCommentTextParams = SetCommentTextParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-72f3a9".to_string(),
        broadcast_id: "news/article/2026/06/19/product-launch".to_string(),
        comment_text_update_request: models::CommentTextUpdateRequest {
            text: "Updated: Congratulations on the launch! Clarified a few points.".to_string(),
        },
        edit_key: Some("edit-key-9f8b".to_string()),
        sso: Some("sso-token-user-abc123".to_string()),
    };
    let response = set_comment_text(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]