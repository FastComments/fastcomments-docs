## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |

## Response

Returns: [`GetVotesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_votes_response.rs)

## Example

[inline-code-attrs-start title = 'get_votes Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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
