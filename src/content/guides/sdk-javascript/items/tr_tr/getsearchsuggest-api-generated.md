---
## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| textSearch | string | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationSuggestResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getSearchSuggest Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-9f8b7c6d";
  const textSearch: string = "harassment";
  const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";
  const suggestResponse: ModerationSuggestResponse = await getSearchSuggest(tenantId, textSearch, sso);
  const minimalResponse: ModerationSuggestResponse = await getSearchSuggest(tenantId);
})();
[inline-code-end]

---