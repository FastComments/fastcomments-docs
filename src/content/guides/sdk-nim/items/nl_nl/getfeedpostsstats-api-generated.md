## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|--------------|
| tenantId | string | Ja |  |
| postIds | seq[string] | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`Option[GetFeedPostsStats_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_feed_posts_stats200response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getFeedPostsStats Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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