## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## Response

Returns: [`Option[GetPagesAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pages_api_response.nim)

## Example

[inline-code-attrs-start title = 'getPages Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (pagesOpt, httpResp) = client.getPages(tenantId = "my-tenant-123")
if pagesOpt.isSome:
  let pages = pagesOpt.get()
  echo pages
else:
  echo "No pages returned"
echo httpResp
[inline-code-end]
