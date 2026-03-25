## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| comment_id | String | はい |  |
| vote_id | String | はい |  |
| url_id | String | はい |  |
| broadcast_id | String | はい |  |
| edit_key | String | いいえ |  |
| sso | String | いいえ |  |

## レスポンス

返却: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_vote_200_response.rs)

## 例

[inline-code-attrs-start title = 'delete_comment_vote の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteCommentVoteParams = DeleteCommentVoteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-12345".to_string(),
        vote_id: "vote-67890".to_string(),
        url_id: "news/world/article-2026".to_string(),
        broadcast_id: "broadcast-1".to_string(),
        edit_key: Some("editkey-abc123".to_string()),
        sso: Some("sso-token-xyz".to_string()),
    };
    let response: DeleteCommentVote200Response = delete_comment_vote(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---