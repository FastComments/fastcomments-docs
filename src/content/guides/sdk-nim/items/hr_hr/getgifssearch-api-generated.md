## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| search | string | Ne |  |
| locale | string | Ne |  |
| rating | string | Ne |  |
| page | float64 | Ne |  |

## Odgovor

Vraća: [`Option[GetGifsSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_gifs_search_response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer getGifsSearch'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getGifsSearch(
  tenantId = "my-tenant-123",
  search = "funny cat",
  locale = "en-US",
  rating = "PG",
  page = 1.0
)

if response.isSome:
  let gifs = response.get()
  echo "Fetched GIFs response:", gifs
[inline-code-end]

---