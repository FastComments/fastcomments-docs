## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| limit | number | Nie |  |
| skip | number | Nie |  |
| order | SORTDIR | Nie |  |
| after | number | Nie |  |
| before | number | Nie |  |

## Odpowiedź

Zwraca: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetAuditLogs200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getAuditLogs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9a8b7c';
const limit: number = 100;
const skip: number = 0;
const after: number = Date.now() - 30 * 24 * 60 * 60 * 1000; // 30 dni temu
const before: number = Date.now();
const auditLogs: GetAuditLogs200Response = await getAuditLogs(tenantId, limit, skip, undefined, after, before);
[inline-code-end]

---