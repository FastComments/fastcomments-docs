## 參數

| Name | Type | 必要 | 說明 |
|------|------|------|-------------|
| tenant_id | String | 是 |  |
| comment_id | String | 是 |  |
| vote_id | String | 是 |  |
| url_id | String | 是 |  |
| broadcast_id | String | 是 |  |
| edit_key | String | 否 |  |
| sso | String | 否 |  |

## 回應

回傳: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_delete_response.rs)

## 範例

[inline-code-attrs-start title = 'delete_comment_vote 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_delete_vote() -> Result<VoteDeleteResponse, Error> {
    let params: DeleteCommentVoteParams = DeleteCommentVoteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-8f3a2b9e".to_string(),
        vote_id: "vote-7d124f".to_string(),
        url_id: "news/politics/2026-election".to_string(),
        broadcast_id: "web-1234".to_string(),
        edit_key: Some("edit-abc123".to_string()),
        sso: Some("sso-token-xyz".to_string()),
    };

    let response: VoteDeleteResponse = delete_comment_vote(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---