## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|-------------|
| tenant_id | String | 是 |  |
| url_id | String | 是 |  |

## 回應

回傳: [`GetVotesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_votes_response.rs)

## 範例

[inline-code-attrs-start title = 'get_votes 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_votes() -> Result<GetVotesResponse, Error> {
    let params: GetVotesParams = GetVotesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/2026/06/product-launch".to_string(),
        page_size: Some(25),
        cursor: Some("cursor_2026_06_ab12".to_string()),
    };
    let votes: GetVotesResponse = get_votes(&configuration, params).await?;
    Ok(votes)
}
[inline-code-end]

---