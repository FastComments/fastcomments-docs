## パラメーター

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| id | String | はい |  |
| updatable_comment_params | models::UpdatableCommentParams | はい |  |
| context_user_id | String | いいえ |  |
| do_spam_check | bool | いいえ |  |
| is_live | bool | いいえ |  |

## レスポンス

返却: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 例

[inline-code-attrs-start title = 'update_comment 例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_update() -> Result<(), Error> {
    let updatable = models::UpdatableCommentParams {
        content: "Edited comment about the latest news article".to_string(),
        ..Default::default()
    };
    let params = UpdateCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "comment-789".to_string(),
        updatable_comment_params: updatable,
        context_user_id: Some("reader-42".to_string()),
        do_spam_check: Some(true),
        is_live: Some(true),
    };
    let _ = update_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]