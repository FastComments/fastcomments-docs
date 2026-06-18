## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| search | string | Ja |  |
| locale | string | Nein |  |
| rating | string | Nein |  |
| page | number | Nein |  |

## Antwort

Gibt zurück: [`GetGifsSearch200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsSearch200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'getGifsSearch Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_9876";
const search: string = "cat playing piano";
const locale: string = "en-US";
const rating: string = "pg";
const page: number = 1;
const result: GetGifsSearch200Response = await getGifsSearch(tenantId, search, locale, rating, page);
[inline-code-end]

---