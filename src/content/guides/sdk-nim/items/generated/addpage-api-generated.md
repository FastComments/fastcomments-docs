## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createAPIPageData | CreateAPIPageData | No |  |

## Response

Returns: [`Option[AddPageAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_page_api_response.nim)

## Example

[inline-code-attrs-start title = 'addPage Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let createData = CreateAPIPageData(path = "news/2026/market-update", title = "Market Update — Jan 2026", tags = @["finance", "markets"], isPublished = true)
let (response, httpResponse) = client.addPage(tenantId = "my-tenant-123", createAPIPageData = createData)
if response.isSome:
  let page = response.get()
  discard page
[inline-code-end]
