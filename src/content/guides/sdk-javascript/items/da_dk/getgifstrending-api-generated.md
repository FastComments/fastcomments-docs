## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| locale | string | Nej |  |
| rating | string | Nej |  |
| page | number | Nej |  |

## Svar

Returnerer: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsTrendingResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getGifsTrending-eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-media';
const locale: string = 'en-US';
const rating: string = 'pg-13';
const page: number = 2;
const trending: GetGifsTrendingResponse = await getGifsTrending(tenantId, locale, rating, page);
[inline-code-end]

---