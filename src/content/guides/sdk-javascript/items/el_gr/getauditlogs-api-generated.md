## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| limit | number | Όχι |  |
| skip | number | Όχι |  |
| order | SORTDIR | Όχι |  |
| after | number | Όχι |  |
| before | number | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetAuditLogsResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getAuditLogs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_87f9a4';
const limit: number = 100;
const skip: number = 0;
const order: SORTDIR = SORTDIR.DESC;
const after: number = Date.now() - 30 * 24 * 60 * 60 * 1000; // πριν από 30 ημέρες
const auditLogsResponse: GetAuditLogsResponse = await getAuditLogs(tenantId, limit, skip, order, after);
console.log((auditLogsResponse as unknown) ? 'Audit logs fetched' : 'No logs');
[inline-code-end]

---