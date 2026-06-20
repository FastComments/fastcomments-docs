Lister sider for en tenant. Bruges af FChat desktop-klienten til at udfylde dens rumliste.
Kræver, at `enableFChat` er true i den løste brugerdefinerede konfiguration for hver side.
Sider, der kræver SSO, bliver filtreret i forhold til den anmodende brugers gruppeadgang.

## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| cursor | string | No |  |
| limit | int | No |  |
| q | string | No |  |
| sortBy | PagesSortBy | No |  |
| hasComments | bool | No |  |

## Svar

Returnerer: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Eksempel

[inline-code-attrs-start title = 'getPagesPublic Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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