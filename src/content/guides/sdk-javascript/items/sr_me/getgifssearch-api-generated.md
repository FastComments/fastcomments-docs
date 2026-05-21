## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| search | string | Da |  |
| locale | string | Ne |  |
| rating | string | Ne |  |
| page | number | Ne |  |

## Odgovor

Vraća: [`GifSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifSearchResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getGifsSearch'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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