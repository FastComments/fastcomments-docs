## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| search | string | Da |  |
| locale | string | Ne |  |
| rating | string | Ne |  |
| page | number | Ne |  |

## Odgovor

Vrne: [`GifSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifSearchResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getGifsSearch'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_fcm_42";
const search: string = "funny golden retriever";
const locale: string = "en-US";
const rating: string = "pg";
const page: number = 2;
const result: GifSearchResponse = await getGifsSearch(tenantId, search, locale, rating, page);
[inline-code-end]

---