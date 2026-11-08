req
tenantId
afterId

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | GetFeedPostsPublicOptions | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`Option[PublicFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_feed_posts_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getFeedPostsPublic उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (feedResponseOpt, httpResponse) = client.getFeedPostsPublic(tenantId = "my-tenant-123", options = GetFeedPostsPublicOptions())
if feedResponseOpt.isSome:
  let feedResponse = feedResponseOpt.get()
[inline-code-end]