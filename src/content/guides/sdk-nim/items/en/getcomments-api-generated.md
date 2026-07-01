## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetCommentsOptions | No |  |

## Response

Returns: [`Option[APIGetCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_comments_response.nim)

## Example

[inline-code-attrs-start title = 'getComments Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let options = GetCommentsOptions(
  urlId = "news/article-title",
  page = 0,
  pageSize = 20,
  sort = "newest",
  includeDeleted = false,
  includeReplies = true,
  userIds = @[],
  tags = @[]
)

let (response, httpResponse) = client.getComments(tenantId = "my-tenant-123", options = options)

if response.isSome:
  let comments = response.get()
  discard comments
[inline-code-end]
