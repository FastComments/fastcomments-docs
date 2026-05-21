## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| search | string | Yes |  |
| locale | string | No |  |
| rating | string | No |  |
| page | number | No |  |

## Risposta

Restituisce: [`GifSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifSearchResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getGifsSearch'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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