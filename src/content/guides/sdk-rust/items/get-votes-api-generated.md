## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |

## Response

Returns: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_votes_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_votes Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetVotesParams = GetVotesParams {
    tenant_id: "acme-corp-tenant".to_string(),
    url_id: "news/acme/2026/01/13/major-update".to_string(),
    cursor: Some("cursor-20260113-01".to_string()),
    limit: Some(100),
};
let votes: GetVotes200Response = get_votes(&configuration, params).await?;
[inline-code-end]
