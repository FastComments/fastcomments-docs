## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| locale | string | Nee |  |
| rating | string | Nee |  |
| page | float64 | Nee |  |

## Respons

Retourneert: [`Option[GetGifsTrendingResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_gifs_trending_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getGifsTrending Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getGifsTrending(tenantId = "my-tenant-123",
  locale = "en-US",
  rating = "pg-13",
  page = 1.0)
if response.isSome:
  let trending = response.get()
[inline-code-end]