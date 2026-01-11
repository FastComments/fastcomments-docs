## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |

## Response

Returns: [`GetPagesApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_pages_api_response.rs)

## Example

[inline-code-attrs-start title = 'get_pages Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetPagesParams = GetPagesParams {
    tenant_id: "acme-corp-tenant".to_string(),
    path: Some("news/article".to_string()),
    include_drafts: Some(false),
    page: Some(1),
    per_page: Some(25),
};

let pages: GetPagesApiResponse = get_pages(&configuration, params).await?;
[inline-code-end]
