Listet Seiten für einen Mandanten. Wird vom FChat-Desktop-Client verwendet, um seine Raumliste zu befüllen.
Erfordert, dass `enableFChat` in der aufgelösten benutzerdefinierten Konfiguration für jede Seite den Wert true hat.
Seiten, die SSO erfordern, werden gegen den Gruppenzugriff des anfragenden Benutzers gefiltert.

## Parameter

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| cursor | string | Nein |  |
| limit | number | Nein |  |
| q | string | Nein |  |
| sortBy | PagesSortBy | Nein |  |
| hasComments | boolean | Nein |  |

## Antwort

Gibt zurück: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getPagesPublic Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f9b2c";
const cursor: string = "cursor_0001a2b3";
const limit: number = 25;
const q: string = "product page";
const hasComments: boolean = true;
const response: GetPublicPagesResponse = await getPagesPublic(tenantId, cursor, limit, q, undefined, hasComments);
[inline-code-end]

---