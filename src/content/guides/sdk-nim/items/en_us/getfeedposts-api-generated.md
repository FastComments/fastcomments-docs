
req
tenantId
afterId

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetFeedPostsOptions | No |  |

## Response

Returns: [`Option[GetFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_feed_posts_response.nim)

## Example

[inline-code-attrs-start title = 'getFeedPosts Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (feedResponseOpt, httpResp) = client.getFeedPosts(tenantId = "my-tenant-123", options = GetFeedPostsOptions())
if feedResponseOpt.isSome:
  let feedResponse = feedResponseOpt.get()
  echo feedResponse
[inline-code-end]
