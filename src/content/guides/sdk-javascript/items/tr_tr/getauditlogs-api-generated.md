## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| limit | number | Hayır |  |
| skip | number | Hayır |  |
| order | SORTDIR | Hayır |  |
| after | number | Hayır |  |
| before | number | Hayır |  |

## Yanıt

Döndürür: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetAuditLogs200Response.ts)

## Örnek

[inline-code-attrs-start title = 'getAuditLogs Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9a8b7c';
const limit: number = 100;
const skip: number = 0;
const after: number = Date.now() - 30 * 24 * 60 * 60 * 1000; // 30 gün önce
const before: number = Date.now();
const auditLogs: GetAuditLogs200Response = await getAuditLogs(tenantId, limit, skip, undefined, after, before);
[inline-code-end]

---