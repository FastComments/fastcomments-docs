Lijst pagina's voor een tenant. Wordt gebruikt door de FChat desktopclient om de kamerlijst te vullen.
Vereist dat `enableFChat` op true staat in de opgeloste aangepaste configuratie voor elke pagina.
Pagina's die SSO vereisen, worden gefilterd op basis van de groepsrechten van de aanvragende gebruiker.

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| cursor | string | No |  |
| limit | number | No |  |
| q | string | No |  |
| sortBy | PagesSortBy | No |  |
| hasComments | boolean | No |  |

## Respons

Retourneert: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getPagesPublic Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f9b2c";
const cursor: string = "cursor_0001a2b3";
const limit: number = 25;
const q: string = "product page";
const hasComments: boolean = true;
const response: GetPublicPagesResponse = await getPagesPublic(tenantId, cursor, limit, q, undefined, hasComments);
[inline-code-end]

---