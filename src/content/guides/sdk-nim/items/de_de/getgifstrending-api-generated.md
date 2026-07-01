## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| options | GetGifsTrendingOptions | Nein |  |

## Antwort

Rückgabe: [`Option[GetGifsTrendingResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_gifs_trending_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getGifsTrending Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResponse, httpResponse) = client.getGifsTrending(
  tenantId = "my-tenant-123",
  options = GetGifsTrendingOptions()
)

if maybeResponse.isSome:
  let gifs = maybeResponse.get()
  echo gifs
[inline-code-end]