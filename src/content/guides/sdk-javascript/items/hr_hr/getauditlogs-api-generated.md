## Parametri

| Naziv | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| limit | number | Ne |  |
| skip | number | Ne |  |
| order | SORTDIR | Ne |  |
| after | number | Ne |  |
| before | number | Ne |  |

## Odgovor

Vraća: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetAuditLogs200Response.ts)

## Primjer

[inline-code-attrs-start title = 'getAuditLogs Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_5f8d7c3a';
const limit: number = 100;
const skip: number = 0;
const order: SORTDIR = 'DESC' as SORTDIR;
const after: number = Date.now() - 7 * 24 * 60 * 60 * 1000; // prije tjedan dana
const before: number = Date.now();
const result: GetAuditLogs200Response = await getAuditLogs(tenantId, limit, skip, order, after, before);
[inline-code-end]

---