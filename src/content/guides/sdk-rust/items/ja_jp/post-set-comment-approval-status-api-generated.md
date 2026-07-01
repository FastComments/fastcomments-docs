## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| comment_id | String | はい |  |
| approved | bool | いいえ |  |
| broadcast_id | String | いいえ |  |
| sso | String | いいえ |  |

## レスポンス

戻り値: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_approved_response.rs)

## 例

[inline-code-attrs-start title = 'post_set_comment_approval_status の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn approve_comment(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = PostSetCommentApprovalStatusParams {
        tenant_id: "acme-corp".to_string(),
        comment_id: "cmt-9876".to_string(),
        approved: Some(true),
        broadcast_id: Some("broadcast-2023".to_string()),
        sso: None,
    };
    let _response = post_set_comment_approval_status(configuration, params).await?;
    Ok(())
}
[inline-code-end]