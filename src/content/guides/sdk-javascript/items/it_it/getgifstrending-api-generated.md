## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| locale | string | No |  |
| rating | string | No |  |
| page | number | No |  |

## Risposta

Restituisce: [`GifSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifSearchResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getGifsTrending'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-tenant-01";
const trendingBasic: GifSearchResponse = await getGifsTrending(tenantId);

const locale: string = "en-GB";
const rating: string = "pg";
const page: number = 1;
const trendingWithOptions: GifSearchResponse = await getGifsTrending(tenantId, locale, rating, page);
[inline-code-end]

---