## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| locale | string | いいえ |  |
| rating | string | いいえ |  |
| page | float64 | いいえ |  |

## レスポンス

戻り値: [`Option[GetGifsTrendingResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_gifs_trending_response.nim)

## 例

[inline-code-attrs-start title = 'getGifsTrending の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getGifsTrending(tenantId = "my-tenant-123",
  locale = "en-US",
  rating = "pg-13",
  page = 1.0)
if response.isSome:
  let trending = response.get()
[inline-code-end]

---