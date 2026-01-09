## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| postIds | seq[string] | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`Option[GetFeedPostsStats_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_feed_posts_stats200response.nim)

## 例

[inline-code-attrs-start title = 'getFeedPostsStats の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getFeedPostsStats(
  tenantId = "my-tenant-123",
  postIds = @["news/article-2025-11-22", "opinion/market-trends-452"],
  sso = ""
)

if response.isSome:
  let stats = response.get()
  discard stats
[inline-code-end]

---