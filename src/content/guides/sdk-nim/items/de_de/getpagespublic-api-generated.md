Listet Seiten für einen Mandanten. Wird vom FChat-Desktop-Client verwendet, um dessen Raumliste zu füllen.
Erfordert, dass `enableFChat` für jede Seite in der aufgelösten Custom-Konfiguration auf true gesetzt ist.
Seiten, die SSO benötigen, werden anhand des Gruppen-Zugriffs des anfragenden Benutzers gefiltert.

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| cursor | string | Nein |  |
| limit | int | Nein |  |
| q | string | Nein |  |
| sortBy | PagesSortBy | Nein |  |
| hasComments | bool | Nein |  |

## Antwort

Gibt zurück: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getPagesPublic Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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