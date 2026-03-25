## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| context_user_id | String | Nein |  |
| is_live | bool | Nein |  |

## Antwort

Gibt zurück: [`DeleteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_200_response.rs)

## Beispiel

[inline-code-attrs-start title = 'delete_comment Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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