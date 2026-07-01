## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| vote_id | String | Yes |  |
| broadcast_id | String | No |  |
| sso | String | No |  |

## 回應

返回：[`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_delete_response.rs)

## 範例

[inline-code-attrs-start title = 'delete_moderation_vote 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = DeleteModerationVoteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article-42".to_string(),
        vote_id: "vote-12345".to_string(),
        broadcast_id: Some("broadcast-987".to_string()),
        sso: None,
    };
    let _response: VoteDeleteResponse = delete_moderation_vote(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---