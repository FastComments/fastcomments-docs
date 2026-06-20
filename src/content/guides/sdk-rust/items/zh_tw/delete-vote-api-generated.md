## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| edit_key | String | 否 |  |

## 回應

回傳: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_delete_response.rs)

## 範例

[inline-code-attrs-start title = 'delete_vote 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteVoteParams = DeleteVoteParams {
        tenant_id: String::from("acme-corp-tenant"),
        id: String::from("article-5678-comment-1234"),
        edit_key: Some(String::from("editkey-9b2f4e")),
    };
    let response: VoteDeleteResponse = delete_vote(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---