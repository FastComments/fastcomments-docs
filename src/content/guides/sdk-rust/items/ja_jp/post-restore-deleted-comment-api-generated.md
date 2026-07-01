## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| comment_id | String | はい |  |
| broadcast_id | String | いいえ |  |
| sso | String | いいえ |  |

## レスポンス

Returns: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 例

[inline-code-attrs-start title = 'post_restore_deleted_comment の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn restore_comment() -> Result<(), Error> {
    let config: &configuration::Configuration = get_configuration();
    let params = PostRestoreDeletedCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "comment-12345".to_string(),
        broadcast_id: Some("broadcast-987".to_string()),
        sso: Some("user@example.com".to_string()),
    };
    let _response = post_restore_deleted_comment(config, params).await?;
    Ok(())
}
[inline-code-end]