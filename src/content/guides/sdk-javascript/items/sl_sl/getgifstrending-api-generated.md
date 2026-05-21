## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| locale | string | Ne |  |
| rating | string | Ne |  |
| page | number | Ne |  |

## Odgovor

Vrača: [`GifSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifSearchResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer uporabe getGifsTrending'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42';
const locale: string = 'en-US';
const rating: string = 'PG';
const page: number = 1;
const result: GifSearchResponse = await getGifsTrending(tenantId, locale, rating, page);
[inline-code-end]

---