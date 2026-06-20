## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| postIds | seq[string] | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`Option[FeedPostsStatsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_feed_posts_stats_response.nim)

## 示例

[inline-code-attrs-start title = 'getFeedPostsStats 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getFeedPostsStats(tenantId = "my-tenant-123", postIds = @["news/article-2026", "opinion/market-trends"], sso = "")
if response.isSome:
  let stats = response.get()
  echo "Received feed posts stats for tenant:", " my-tenant-123"
  discard stats
[inline-code-end]

---