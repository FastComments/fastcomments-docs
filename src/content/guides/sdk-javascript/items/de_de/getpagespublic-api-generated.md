Listet Seiten für einen Mandanten. Wird vom FChat-Desktop-Client verwendet, um dessen Raumliste zu füllen.
`enableFChat` muss in der aufgelösten benutzerdefinierten Konfiguration für jede Seite auf true gesetzt sein.
Seiten, die SSO erfordern, werden anhand der Gruppenberechtigungen des anfragenden Benutzers gefiltert.

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| cursor | string | Nein |  |
| limit | number | Nein |  |
| q | string | Nein |  |
| sortBy | PagesSortBy | Nein |  |
| hasComments | boolean | Nein |  |

## Antwort

Gibt zurück: [`GetPagesPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublic200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'getPagesPublic Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3b2c';
const cursor: string = 'eyJwYWdlIjoiMTIwIn0';
const limit: number = 25;
const q: string = 'homepage hero';
const hasComments: boolean = true;

const response: GetPagesPublic200Response = await getPagesPublic(
  tenantId,
  cursor,
  limit,
  q,
  undefined,
  hasComments
);
[inline-code-end]

---