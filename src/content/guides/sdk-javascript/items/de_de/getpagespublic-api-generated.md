Listet Seiten für einen Tenant auf. Wird vom FChat-Desktop-Client verwendet, um seine Raumliste zu füllen.
Erfordert, dass `enableFChat` in der aufgelösten benutzerdefinierten Konfiguration für jede Seite auf true gesetzt ist.
Seiten, die SSO erfordern, werden anhand des Gruppenzugriffs des anfragenden Benutzers gefiltert.

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

Gibt zurück: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getPagesPublic Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPublicPages() {
  const tenantId: string = "tenant_12345";
  const cursor: string = "page_5";
  const limit: number = 20;
  const query: string = "support";
  const hasComments: boolean = true;

  const response: GetPublicPagesResponse = await getPagesPublic(
    tenantId,
    cursor,
    limit,
    query,
    undefined,
    hasComments
  );

  console.log(response);
}
[inline-code-end]