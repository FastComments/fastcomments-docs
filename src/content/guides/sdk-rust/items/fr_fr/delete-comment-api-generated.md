## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| id | String | Oui |  |
| context_user_id | String | Non |  |
| is_live | bool | Non |  |

## Réponse

Renvoie : [`DeleteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_200_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de delete_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete_comment() -> Result<DeleteComment200Response, Error> {
    let params: DeleteCommentParams = DeleteCommentParams {
        tenant_id: "acme-newsroom".to_string(),
        id: "news/article-2026/comments/abc123".to_string(),
        context_user_id: Some("user-789".to_string()),
        is_live: Some(true),
    };
    let response: DeleteComment200Response = delete_comment(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---