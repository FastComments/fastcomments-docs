## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |

## Yanıt

Döndürür: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_votes_200_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_votes Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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