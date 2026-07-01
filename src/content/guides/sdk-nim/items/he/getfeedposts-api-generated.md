req
tenantId
afterId

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| options | GetFeedPostsOptions | לא |  |

## תגובה

מחזיר: [`Option[GetFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_feed_posts_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getFeedPosts'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (feedResponseOpt, httpResp) = client.getFeedPosts(tenantId = "my-tenant-123", options = GetFeedPostsOptions())
if feedResponseOpt.isSome:
  let feedResponse = feedResponseOpt.get()
  echo feedResponse
[inline-code-end]