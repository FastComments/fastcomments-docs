## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |

## Response

Returns: [`GetPageByUrlidApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_page_by_urlid_api_response.rs)

## Example

[inline-code-attrs-start title = 'get_page_by_urlid Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetPageByUrlidParams = GetPageByUrlidParams {
    tenant_id: String::from("acme-corp-tenant"),
    url_id: String::from("news/2025/launch-announcement"),
    include_comments: Some(true),
};
let page: GetPageByUrlidApiResponse = get_page_by_urlid(&configuration, params).await?;
[inline-code-end]
