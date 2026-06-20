---
Prikazuje listu stranica za tenant. Koristi se u FChat desktop klijentu za popunjavanje liste soba.
Zahteva da `enableFChat` bude postavljeno na true u rešavanom prilagođenom podešavanju za svaku stranicu.
Stranice koje zahtevaju SSO filtriraju se u skladu sa pristupom grupa korisnika koji podnosi zahtev.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| cursor | string | Ne |  |
| limit | int | Ne |  |
| q | string | Ne |  |
| sortBy | PagesSortBy | Ne |  |
| hasComments | bool | Ne |  |

## Odgovor

Vraća: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Primer

[inline-code-attrs-start title = 'getPagesPublic Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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