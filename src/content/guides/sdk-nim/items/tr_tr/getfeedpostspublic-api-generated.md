req
tenantId
afterId

## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetFeedPostsPublicOptions | No |  |

## Yanıt

Returns: [`Option[PublicFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_feed_posts_response.nim)

## Örnek

[inline-code-attrs-start title = 'getFeedPostsPublic Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (feedResponseOpt, httpResponse) = client.getFeedPostsPublic(tenantId = "my-tenant-123", options = GetFeedPostsPublicOptions())
if feedResponseOpt.isSome:
  let feedResponse = feedResponseOpt.get()
[inline-code-end]