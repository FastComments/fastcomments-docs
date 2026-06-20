## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| id | String | Sì |  |
| context_user_id | String | No |  |
| is_live | bool | No |  |

## Risposta

Restituisce: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_result.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di delete_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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