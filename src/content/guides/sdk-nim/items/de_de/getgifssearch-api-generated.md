## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| search | string | Nein |  |
| locale | string | Nein |  |
| rating | string | Nein |  |
| page | float64 | Nein |  |

## Antwort

Gibt zurück: [`Option[GetGifsSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_gifs_search_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getGifsSearch Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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