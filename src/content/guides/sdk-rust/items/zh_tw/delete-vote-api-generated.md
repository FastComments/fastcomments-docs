## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| edit_key | String | 否 |  |

## 回應

回傳: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_vote_200_response.rs)

## 範例

[inline-code-attrs-start title = 'delete_vote 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn delete_vote_example() -> Result<DeleteCommentVote200Response, Error> {
    let params: DeleteVoteParams = DeleteVoteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "comment-98765".to_string(),
        edit_key: Some("edit-4f2b9c".to_string()),
    };
    let response: DeleteCommentVote200Response = delete_vote(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---