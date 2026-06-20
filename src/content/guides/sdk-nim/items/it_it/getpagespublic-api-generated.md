---
Elenca le pagine per un tenant. Utilizzato dal client desktop FChat per popolare l'elenco delle sue stanze.
Richiede che `enableFChat` sia true nella configurazione personalizzata risolta per ciascuna pagina.
Le pagine che richiedono SSO vengono filtrate rispetto ai gruppi di accesso dell'utente che effettua la richiesta.

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| cursor | string | No |  |
| limit | int | No |  |
| q | string | No |  |
| sortBy | PagesSortBy | No |  |
| hasComments | bool | No |  |

## Risposta

Restituisce: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio getPagesPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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