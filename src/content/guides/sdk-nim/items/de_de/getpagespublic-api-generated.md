Seiten für einen Mandanten auflisten. Wird vom FChat-Desktop-Client verwendet, um dessen Raumliste zu füllen.  
Erfordert, dass `enableFChat` in der aufgelösten benutzerdefinierten Konfiguration für jede Seite auf true gesetzt ist.  
Seiten, die SSO benötigen, werden anhand des Gruppen‑Zugriffs des anfordernden Benutzers gefiltert.

## Parameters

| Name   | Typ    | Erforderlich | Beschreibung |
|--------|--------|--------------|--------------|
| tenantId | string | Ja           |  |
| options  | GetPagesPublicOptions | Nein |  |

## Response

Rückgabe: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getPagesPublic Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(tenantId = "my-tenant-123", options = GetPagesPublicOptions())
if response.isSome:
  let pages = response.get()
  echo pages
[inline-code-end]

---