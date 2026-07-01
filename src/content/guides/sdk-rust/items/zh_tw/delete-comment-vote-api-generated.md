## 參數

| 名稱 | 型別 | 必填 | 描述 |
|------|------|------|------|
| tenant_id | String | 是 |  |
| comment_id | String | 是 |  |
| vote_id | String | 是 |  |
| url_id | String | 是 |  |
| broadcast_id | String | 是 |  |
| edit_key | String | 否 |  |
| sso | String | 否 |  |

## 回應

返回: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_delete_response.rs)

## 範例

[inline-code-attrs-start title = 'delete_comment_vote 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
#[tokio::main]
async fn main() -> Result<(), Error> {
    let params = DeleteCommentVoteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "comment-12345".to_string(),
        vote_id: "vote-67890".to_string(),
        url_id: "news/article".to_string(),
        broadcast_id: "broadcast-abc".to_string(),
        edit_key: Some("edit-key-xyz".to_string()),
        sso: Some("sso-token-123".to_string()),
    };
    let _response: VoteDeleteResponse = delete_comment_vote(&configuration, params).await?;
    Ok(())
}
[inline-code-end]