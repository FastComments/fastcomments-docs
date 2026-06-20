## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| id | String | Oui |  |
| context_user_id | String | Non |  |
| is_live | bool | Non |  |

## Réponse

Renvoie : [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_result.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de delete_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete() -> Result<DeleteCommentResult, Error> {
    let params: DeleteCommentParams = DeleteCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "comment-6f8a21b4".to_string(),
        context_user_id: Some("editor-42".to_string()),
        is_live: Some(true),
    };
    let deleted: DeleteCommentResult = delete_comment(&configuration, params).await?;
    Ok(deleted)
}
[inline-code-end]

---