## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| locale | string | Ne |  |
| rating | string | Ne |  |
| page | float64 | Ne |  |

## Odgovor

Vraća: [`Option[GetGifsTrendingResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_gifs_trending_response.nim)

## Primjer

[inline-code-attrs-start title = 'getGifsTrending Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getGifsTrending(tenantId = "my-tenant-123",
  locale = "en-US",
  rating = "pg-13",
  page = 1.0)
if response.isSome:
  let trending = response.get()
[inline-code-end]

---