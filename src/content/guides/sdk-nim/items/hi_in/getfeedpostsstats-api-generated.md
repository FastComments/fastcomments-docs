## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| postIds | seq[string] | नहीं |  |
| sso | string = "" | नहीं |  |

## प्रतिक्रिया

रिटर्न: [`Option[FeedPostsStatsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_feed_posts_stats_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getFeedPostsStats उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (statsOpt, httpResponse) = client.getFeedPostsStats(
  tenantId = "my-tenant-123",
  postIds = @["post-abc", "post-def"],
  sso = "sso-token",
)

if statsOpt.isSome:
  let stats = statsOpt.get()
  # stats का आवश्यकतानुसार उपयोग करें


[inline-code-end]