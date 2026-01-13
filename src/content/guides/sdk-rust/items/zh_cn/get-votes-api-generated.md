## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| url_id | String | 是 |  |

## 响应

返回：[`GetVotes200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_votes_200_response.rs)

## 示例

[inline-code-attrs-start title = 'get_votes 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_votes_example() -> Result<(), Error> {
    let params: GetVotesParams = GetVotesParams {
        tenant_id: String::from("acme-corp-tenant"),
        url_id: String::from("news/article/2026-01-12/housing-market"),
    };
    let votes: GetVotes200Response = get_votes(&configuration, params).await?;
    let _ = votes;
    Ok(())
}
[inline-code-end]

---