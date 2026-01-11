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
let (response, httpResponse) = client.addPage(tenantId = "my-tenant-123", createAPIPageData = CreateAPIPageData())
if response.isSome:
  let addedPage = response.get()
  discard addedPage
[inline-code-end]
