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
let (response, httpResponse) = client.patchPage(tenantId = "my-tenant-123",
  id = "news/city-council-plan",
  updateAPIPageData = UpdateAPIPageData(title = "City Council Approves New Plan",
    urlId = "news/city-council-plan",
    enabled = true,
    allowComments = true,
    tags = @["local", "politics"],
    sortOrder = 1))

if response.isSome:
  let updatedPage = response.get()
  echo "Updated page received"
[inline-code-end]
