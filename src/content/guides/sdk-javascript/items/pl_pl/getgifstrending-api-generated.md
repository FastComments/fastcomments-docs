## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| locale | string | Nie |  |
| rating | string | Nie |  |
| page | number | Nie |  |

## Odpowiedź

Zwraca: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsTrendingResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getGifsTrending'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-media';
const locale: string = 'en-US';
const rating: string = 'pg-13';
const page: number = 2;
const trending: GetGifsTrendingResponse = await getGifsTrending(tenantId, locale, rating, page);
[inline-code-end]

---