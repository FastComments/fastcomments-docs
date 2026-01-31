## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| page | int | No |  |
| limit | int | No |  |
| skip | int | No |  |
| asTree | bool | No |  |
| skipChildren | int | No |  |
| limitChildren | int | No |  |
| maxTreeDepth | int | No |  |
| urlId | string | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |
| contextUserId | string | No |  |
| hashTag | string | No |  |
| parentId | string | No |  |
| direction | SortDirections | No |  |

## Response

Returns: [`Option[GetComments_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments200response.nim)

## Example

[inline-code-attrs-start title = 'getComments Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getComments(
  tenantId = "my-tenant-123",
  page = 1,
  limit = 20,
  skip = 0,
  asTree = true,
  skipChildren = 0,
  limitChildren = 5,
  maxTreeDepth = 3,
  urlId = "news/article-2026-01",
  userId = "user-789",
  anonUserId = "",
  contextUserId = "",
  hashTag = "breaking",
  parentId = "",
  direction = SortDirections.desc
)

if response.isSome:
  let comments = response.get()
  discard comments
[inline-code-end]
