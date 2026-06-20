## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| context_user_id | String | Nej |  |
| is_live | bool | Nej |  |

## Respons

Returnerer: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_result.rs)

## Eksempel

[inline-code-attrs-start title = 'delete_comment Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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