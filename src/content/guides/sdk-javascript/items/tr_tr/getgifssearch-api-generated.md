## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| search | string | Evet |  |
| locale | string | Hayır |  |
| rating | string | Hayır |  |
| page | number | Hayır |  |

## Yanıt

Döndürür: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsSearchResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getGifsSearch Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_9876";
  const search: string = "celebration fireworks";
  const locale: string = "fr-FR";
  const rating: string = "g";
  const page: number = 1;

  const fullResult: GetGifsSearchResponse = await getGifsSearch(tenantId, search, locale, rating, page);
  const minimalResult: GetGifsSearchResponse = await getGifsSearch(tenantId, "dog memes");
}

runExample();
[inline-code-end]

---