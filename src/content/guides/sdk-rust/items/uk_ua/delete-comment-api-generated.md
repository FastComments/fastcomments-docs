## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| id | String | Так |  |
| context_user_id | String | Ні |  |
| is_live | bool | Ні |  |

## Відповідь

Повертає: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_result.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад delete_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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