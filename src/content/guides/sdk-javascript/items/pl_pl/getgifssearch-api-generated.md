## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| search | string | Tak |  |
| locale | string | Nie |  |
| rating | string | Nie |  |
| page | number | Nie |  |

## Odpowiedź

Zwraca: [`GetGifsSearch200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsSearch200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getGifsSearch'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_9876";
const search: string = "cat playing piano";
const locale: string = "en-US";
const rating: string = "pg";
const page: number = 1;
const result: GetGifsSearch200Response = await getGifsSearch(tenantId, search, locale, rating, page);
[inline-code-end]

---