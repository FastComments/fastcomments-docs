## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## Response

Returns: [`Option[GetPagesAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pages_api_response.nim)

## Example

[inline-code-attrs-start title = 'getPages Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPages(tenantId = "my-tenant-123")
if response.isSome:
  let pages = response.get()
  echo "Pages received for tenant my-tenant-123"
  echo pages
else:
  echo "No pages found, HTTP status: ", $httpResponse.status
[inline-code-end]
