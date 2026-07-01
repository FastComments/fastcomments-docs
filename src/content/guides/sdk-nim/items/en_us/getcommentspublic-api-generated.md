
req
tenantId
urlId

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | GetCommentsPublicOptions | No |  |

## Response

Returns: [`Option[GetCommentsResponseWithPresencePublicComment]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_response_with_presence_public_comment.nim)

## Example

[inline-code-attrs-start title = 'getCommentsPublic Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.getCommentsPublic(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  options = GetCommentsPublicOptions(
    page = 0,
    pageSize = 10,
    includeDeleted = false,
    tags = @[]
  )
)

if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]
