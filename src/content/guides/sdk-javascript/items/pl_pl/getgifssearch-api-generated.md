## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| search | string | Tak |  |
| locale | string | Nie |  |
| rating | string | Nie |  |
| page | number | Nie |  |

## Odpowiedź

Zwraca: [`GifSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifSearchResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia getGifsSearch'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "global-media";
  const search: string = "laughing baby";
  const locale: string = "en-US";
  const rating: string = "pg";
  const page: number = 2;
  const result: GifSearchResponse = await getGifsSearch(tenantId, search, locale, rating, page);
  console.log(result);
})();
[inline-code-end]

---