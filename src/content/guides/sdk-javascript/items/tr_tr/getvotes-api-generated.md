## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |

## Yanıt

Döndürür: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotes200Response.ts)

## Örnek

[inline-code-attrs-start title = 'getVotes Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f8b3c_prod';
const urlId: string = '/news/2026/typescript-ecosystem-update';
const votes: GetVotes200Response = await getVotes(tenantId, urlId);
// Eğer bir isteğe bağlı parametre olsaydı, ör. includeHidden, şu şekilde kullanılabilirdi:
// const votesWithHidden: GetVotes200Response = await getVotes(tenantId, urlId, { includeHidden: true });
[inline-code-end]

---