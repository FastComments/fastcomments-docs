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
let (response, httpResponse) = client.getFeedPostsStats(tenantId = "my-tenant-123", postIds = @["news/2025/election-coverage", "blog/tech/ai-trends"], sso = "")
if response.isSome:
  let stats = response.get()
  echo "Fetched feed posts stats; HTTP status: ", httpResponse.status
else:
  echo "No stats returned; HTTP status: ", httpResponse.status
[inline-code-end]
