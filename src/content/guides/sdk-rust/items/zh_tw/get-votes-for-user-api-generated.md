## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| url_id | String | 是 |  |
| user_id | String | 否 |  |
| anon_user_id | String | 否 |  |

## 回應

回傳：[`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_votes_for_user_response.rs)

## 範例

[inline-code-attrs-start title = 'get_votes_for_user 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_votes_for_user() -> Result<(), Error> {
    let params: GetVotesForUserParams = GetVotesForUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/2026/06/15/market-update".to_string(),
        user_id: Some("user_98765".to_string()),
        anon_user_id: Some("anon-4f3b2a".to_string()),
    };
    let response: GetVotesForUserResponse = get_votes_for_user(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---