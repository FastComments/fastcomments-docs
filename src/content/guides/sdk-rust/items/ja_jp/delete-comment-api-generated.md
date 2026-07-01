## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| context_user_id | String | No |  |
| is_live | bool | No |  |

## レスポンス

戻り値: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_result.rs)

## 例

[inline-code-attrs-start title = 'delete_comment 例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn main() -> Result<(), Error> {
    let params = DeleteCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "comment-12345".to_string(),
        context_user_id: Some("user-6789".to_string()),
        is_live: Some(true),
    };
    let _result = delete_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]