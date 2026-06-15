Lijst met pagina's voor een tenant. Gebruikt door de FChat-desktopclient om zijn kamerlijst te vullen.
Vereist dat `enableFChat` de waarde `true` heeft in de opgeloste aangepaste configuratie voor elke pagina.
Pagina's die SSO vereisen worden gefilterd op basis van de groepsrechten van de aanvragende gebruiker.

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| cursor | string | Nee |  |
| limit | number | Nee |  |
| q | string | Nee |  |
| sortBy | PagesSortBy | Nee |  |
| hasComments | boolean | Nee |  |

## Antwoord

Retourneert: [`GetPagesPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublic200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getPagesPublic Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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