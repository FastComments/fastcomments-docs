## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updateAPIPageData | UpdateAPIPageData | No |  |

## Response

Returns: [`Option[PatchPageAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_patch_page_api_response.nim)

## Example

[inline-code-attrs-start title = 'patchPage Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateData = UpdateAPIPageData(
  title = "Main St Reopens After Repairs",
  path = "news/main-st-reopens",
  description = "Main Street reopened to traffic following overnight repairs",
  isEnabled = true,
  tags = @["local", "infrastructure"]
)

let (response, httpResponse) = client.patchPage(tenantId = "my-tenant-123", id = "page-456", updateAPIPageData = updateData)

if response.isSome:
  let page = response.get()
  echo "Patched page:", $page
else:
  echo "Patch failed, HTTP status: ", httpResponse.status
[inline-code-end]
