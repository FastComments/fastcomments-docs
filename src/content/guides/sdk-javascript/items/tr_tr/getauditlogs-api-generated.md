## Parametreler

| Name | Type | Required | Description |
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
const tenantId: string = 'tenant_5f8d7c3a';
const limit: number = 100;
const skip: number = 0;
const order: SORTDIR = 'DESC' as SORTDIR;
const after: number = Date.now() - 7 * 24 * 60 * 60 * 1000; // bir hafta önce
const before: number = Date.now();
const result: GetAuditLogs200Response = await getAuditLogs(tenantId, limit, skip, order, after, before);
[inline-code-end]

---