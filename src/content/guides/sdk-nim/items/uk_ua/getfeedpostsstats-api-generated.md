## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| postIds | seq[string] | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`Option[FeedPostsStatsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_feed_posts_stats_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getFeedPostsStats'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getFeedPostsStats(tenantId = "my-tenant-123", postIds = @["news/article-2026", "opinion/market-trends"], sso = "")
if response.isSome:
  let stats = response.get()
  echo "Received feed posts stats for tenant:", " my-tenant-123"
  discard stats
[inline-code-end]

---