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
  title = "Homepage — Daily Times",
  slug = "news/homepage",
  enabled = true,
  tags = @["featured", "editorial"],
  description = "Updated summary for the homepage",
  priority = 0
)
let (response, httpResponse) = client.patchPage(tenantId = "my-tenant-123", id = "page-homepage-456", updateAPIPageData = updateData)
if response.isSome:
  let page = response.get()
  echo "Updated page: ", page.id
else:
  echo "No response body, status: ", httpResponse.status
[inline-code-end]
