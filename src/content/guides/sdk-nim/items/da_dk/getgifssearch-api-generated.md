## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| search | string | Nej |  |
| locale | string | Nej |  |
| rating | string | Nej |  |
| page | float64 | Nej |  |

## Svar

Returnerer: [`Option[GetGifsSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_gifs_search_response.nim)

## Eksempel

[inline-code-attrs-start title = 'getGifsSearch Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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