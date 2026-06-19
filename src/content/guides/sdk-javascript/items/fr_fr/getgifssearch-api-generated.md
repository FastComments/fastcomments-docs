## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| search | string | Oui |  |
| locale | string | Non |  |
| rating | string | Non |  |
| page | number | Non |  |

## Réponse

Renvoie : [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsSearchResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getGifsSearch'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-42';
const search: string = 'golden hour sunset';
const locale: string = 'en-US';
const rating: string = 'pg';
const page: number = 1;
const result: GetGifsSearchResponse = await getGifsSearch(tenantId, search, locale, rating, page);
[inline-code-end]

---