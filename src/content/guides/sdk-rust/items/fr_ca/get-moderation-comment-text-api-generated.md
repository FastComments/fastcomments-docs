## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| comment_id | String | Oui |  |
| sso | String | Non |  |

## Réponse

Renvoie : [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_text_response.rs)

## Exemple

[inline-code-attrs-start title = 'get_moderation_comment_text Exemple'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetModerationCommentTextParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "comment-12345".to_string(),
        sso: Some("user-sso-token".to_string()),
    };
    let _response: GetCommentTextResponse =
        get_moderation_comment_text(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---