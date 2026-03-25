## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| url_id | String | はい |  |

## レスポンス

戻り値: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_votes_200_response.rs)

## 例

[inline-code-attrs-start title = 'get_votes の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_votes() -> Result<GetVotes200Response, Error> {
    let params: GetVotesParams = GetVotesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article/2026/03/25/breaking-story".to_string(),
        include_replies: Some(true),
    };
    let votes: GetVotes200Response = get_votes(&configuration, params).await?;
    Ok(votes)
}
[inline-code-end]

---