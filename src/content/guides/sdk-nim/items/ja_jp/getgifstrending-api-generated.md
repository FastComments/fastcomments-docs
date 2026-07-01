---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| options | GetGifsTrendingOptions | いいえ |  |

## レスポンス

戻り値: [`Option[GetGifsTrendingResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_gifs_trending_response.nim)

## 例

[inline-code-attrs-start title = 'getGifsTrending の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResponse, httpResponse) = client.getGifsTrending(
  tenantId = "my-tenant-123",
  options = GetGifsTrendingOptions()
)

if maybeResponse.isSome:
  let gifs = maybeResponse.get()
  echo gifs
[inline-code-end]

---