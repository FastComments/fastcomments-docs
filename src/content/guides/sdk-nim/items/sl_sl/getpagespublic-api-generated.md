---
Seznam strani za najemnika. Uporablja ga namizni odjemalec FChat za napolnitev svojega seznama sob. Zahteva, da ima `enableFChat` vrednost true v razrešeni prilagojeni konfiguraciji za vsako stran. Strani, ki zahtevajo SSO, so filtrirane glede na dostop skupin uporabnika, ki poizveduje.

## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| cursor | string | Ne |  |
| limit | int | Ne |  |
| q | string | Ne |  |
| sortBy | PagesSortBy | Ne |  |
| hasComments | bool | Ne |  |

## Odgovor

Vrne: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getPagesPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(
  tenantId = "my-tenant-123",
  cursor = "",
  limit = 0,
  q = "",
  sortBy = PagesSortBy(0),
  hasComments = false
)

if response.isSome:
  let pages = response.get()
  echo "Retrieved public pages: ", $pages
else:
  echo "No pages returned, HTTP status: ", $httpResponse.status
[inline-code-end]

---