## Parametreler

| Ad   | Tür     | Gerekli | Açıklama |
|------|----------|----------|-------------|
| page | number | No |  |
| count | number | No |  |
| textSearch | string | No |  |
| byIPFromComment | string | No |  |
| filters | string | No |  |
| searchFilters | string | No |  |
| sorts | string | No |  |
| demo | boolean | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Yanıt

Döndürür: [`GetApiCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetApiCommentsResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getApiComments Örnek'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadComments() {
  const fullResult: GetApiCommentsResponse = await getApiComments(
    2,                     // sayfa
    25,                    // adet
    "feedback",           // metinArama
    "192.168.1.100",      // IP'denYorum
    "approved",           // filtreler
    "hasReplies",         // aramaFiltreleri
    "dateDesc",           // sıralama
    false,                // demo
    "tenant-abc123",      // tenantId
    "sso-token-xyz"       // sso
  );

  const minimalResult: GetApiCommentsResponse = await getApiComments(undefined, 5);
}
[inline-code-end]