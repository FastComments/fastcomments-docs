req
tenantId
afterId

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | GetFeedPostsOptions | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[GetFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_feed_posts_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getFeedPosts उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (feedResponseOpt, httpResp) = client.getFeedPosts(tenantId = "my-tenant-123", options = GetFeedPostsOptions())
if feedResponseOpt.isSome:
  let feedResponse = feedResponseOpt.get()
  echo feedResponse
[inline-code-end]