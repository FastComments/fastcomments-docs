## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| sso | String | No |  |

## Réponse

Renvoie : [`ModerationApiChildCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_child_comments_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple get_comment_children'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_children(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetCommentChildrenParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article/comment-9876".to_string(),
        sso: Some("user-42".to_string()),
    };
    let _response: ModerationApiChildCommentsResponse = get_comment_children(config, params).await?;
    Ok(())
}
[inline-code-end]