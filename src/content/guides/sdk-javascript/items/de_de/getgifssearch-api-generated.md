## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| search | string | Ja |  |
| locale | string | Nein |  |
| rating | string | Nein |  |
| page | number | Nein |  |

## Antwort

Gibt zurück: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsSearchResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getGifsSearch'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-42';
const search: string = 'golden hour sunset';
const locale: string = 'en-US';
const rating: string = 'pg';
const page: number = 1;
const result: GetGifsSearchResponse = await getGifsSearch(tenantId, search, locale, rating, page);
[inline-code-end]

---