## 參數

| 名稱 | 類型 | 必需 | 說明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetGifsTrendingOptions | No |  |

## 回應

返回：[`Option[GetGifsTrendingResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_gifs_trending_response.nim)

## 範例

[inline-code-attrs-start title = 'getGifsTrending 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResponse, httpResponse) = client.getGifsTrending(
  tenantId = "my-tenant-123",
  options = GetGifsTrendingOptions()
)

if maybeResponse.isSome:
  let gifs = maybeResponse.get()
  echo gifs
[inline-code-end]