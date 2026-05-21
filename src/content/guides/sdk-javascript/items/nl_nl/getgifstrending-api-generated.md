## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| locale | string | Nee |  |
| rating | string | Nee |  |
| page | number | Nee |  |

## Antwoord

Geeft terug: [`GifSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifSearchResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getGifsTrending Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42';
const locale: string = 'en-US';
const rating: string = 'PG';
const page: number = 1;
const result: GifSearchResponse = await getGifsTrending(tenantId, locale, rating, page);
[inline-code-end]

---