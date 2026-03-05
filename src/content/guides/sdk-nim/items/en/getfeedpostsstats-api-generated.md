## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postIds | seq[string] | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[GetFeedPostsStats_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_feed_posts_stats200response.nim)

## Example

[inline-code-attrs-start title = 'getFeedPostsStats Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getFeedPostsStats(tenantId = "my-tenant-123", postIds = @["news/article-456", "blog/post-789"], sso = "user-abc-sso-token")
if response.isSome:
  let stats = response.get()
  echo "Received feed posts stats: ", stats
else:
  echo "No feed posts stats returned"
[inline-code-end]
