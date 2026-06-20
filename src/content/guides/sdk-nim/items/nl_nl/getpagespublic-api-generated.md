Lijst met pagina's voor een tenant. Gebruikt door de FChat-desktopclient om de lijst met kamers te vullen.
Vereist dat `enableFChat` op true staat in de opgeloste aangepaste configuratie voor elke pagina.
Pagina's die SSO vereisen worden gefilterd op basis van de groepsrechten van de aanvragende gebruiker.

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| cursor | string | Nee |  |
| limit | int | Nee |  |
| q | string | Nee |  |
| sortBy | PagesSortBy | Nee |  |
| hasComments | bool | Nee |  |

## Antwoord

Retourneert: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getPagesPublic Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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