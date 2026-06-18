## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| search | string | Oui |  |
| locale | string | Non |  |
| rating | string | Non |  |
| page | number | Non |  |

## Réponse

Renvoie : [`GetGifsSearch200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsSearch200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple d\'utilisation de getGifsSearch'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_9876";
const search: string = "cat playing piano";
const locale: string = "en-US";
const rating: string = "pg";
const page: number = 1;
const result: GetGifsSearch200Response = await getGifsSearch(tenantId, search, locale, rating, page);
[inline-code-end]

---