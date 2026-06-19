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

Zwraca: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetAuditLogsResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getAuditLogs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_87f9a4';
const limit: number = 100;
const skip: number = 0;
const order: SORTDIR = SORTDIR.DESC;
const after: number = Date.now() - 30 * 24 * 60 * 60 * 1000; // 30 dni temu
const auditLogsResponse: GetAuditLogsResponse = await getAuditLogs(tenantId, limit, skip, order, after);
console.log((auditLogsResponse as unknown) ? 'Audit logs fetched' : 'No logs');
[inline-code-end]

---