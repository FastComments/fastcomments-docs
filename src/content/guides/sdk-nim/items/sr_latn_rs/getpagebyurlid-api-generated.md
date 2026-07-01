## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Response

Vraća: [`Option[GetPageByURLIdAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_page_by_urlid_api_response.nim)

## Example

[inline-code-attrs-start title = 'getPageByURLId Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (pageOpt, httpRes) = client.getPageByURLId(tenantId = "my-tenant-123", urlId = "news/article-title")
if pageOpt.isSome:
  let page = pageOpt.get()
  echo page
[inline-code-end]