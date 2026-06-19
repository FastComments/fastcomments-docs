## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| limit | number | Ні |  |
| skip | number | Ні |  |
| order | SORTDIR | Ні |  |
| after | number | Ні |  |
| before | number | Ні |  |

## Відповідь

Повертає: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetAuditLogsResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getAuditLogs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_87f9a4';
const limit: number = 100;
const skip: number = 0;
const order: SORTDIR = SORTDIR.DESC;
const after: number = Date.now() - 30 * 24 * 60 * 60 * 1000; // 30 днів тому
const auditLogsResponse: GetAuditLogsResponse = await getAuditLogs(tenantId, limit, skip, order, after);
console.log((auditLogsResponse as unknown) ? 'Audit logs fetched' : 'No logs');
[inline-code-end]

---