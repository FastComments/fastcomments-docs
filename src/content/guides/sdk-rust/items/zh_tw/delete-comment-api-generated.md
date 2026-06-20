## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| context_user_id | String | 否 |  |
| is_live | bool | 否 |  |

## 回應

回傳: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_result.rs)

## 範例

[inline-code-attrs-start title = 'delete_comment 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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