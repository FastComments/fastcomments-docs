## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postIds | seq[string] | No |  |
| sso | string | No |  |

## Svar

Returnerer: [`Option[FeedPostsStatsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_feed_posts_stats_response.nim)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på getFeedPostsStats'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getFeedPostsStats(tenantId = "my-tenant-123", postIds = @["news/article-2026", "opinion/market-trends"], sso = "")
if response.isSome:
  let stats = response.get()
  echo "Received feed posts stats for tenant:", " my-tenant-123"
  discard stats
[inline-code-end]

---