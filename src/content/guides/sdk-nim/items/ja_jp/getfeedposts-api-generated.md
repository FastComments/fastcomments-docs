req
tenantId
afterId

## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| options | GetFeedPostsOptions | いいえ |  |

## レスポンス

戻り値: [`Option[GetFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_feed_posts_response.nim)

## 例

[inline-code-attrs-start title = 'getFeedPosts の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (feedResponseOpt, httpResp) = client.getFeedPosts(tenantId = "my-tenant-123", options = GetFeedPostsOptions())
if feedResponseOpt.isSome:
  let feedResponse = feedResponseOpt.get()
  echo feedResponse
[inline-code-end]