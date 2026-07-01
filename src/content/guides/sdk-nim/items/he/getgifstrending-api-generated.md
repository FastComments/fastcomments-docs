## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetGifsTrendingOptions | No |  |

## תגובה

מחזיר: [`Option[GetGifsTrendingResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_gifs_trending_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getGifsTrending'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResponse, httpResponse) = client.getGifsTrending(
  tenantId = "my-tenant-123",
  options = GetGifsTrendingOptions()
)

if maybeResponse.isSome:
  let gifs = maybeResponse.get()
  echo gifs
[inline-code-end]