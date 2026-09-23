## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| limit | number | Не |  |
| skip | number | Не |  |
| order | SORTDIR | Не |  |
| after | number | Не |  |
| before | number | Не |  |
| username | string | Не |  |
| ip | string | Не |  |
| crudType | string | Не |  |
| resourceName | string | Не |  |
| targetId | string | Не |  |
| target | string | Не |  |
| includeManagedTenants | boolean | Не |  |

## Отговор

Връща: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetAuditLogsResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getAuditLogs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchAuditLogs(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const limit: number = 50;
  const skip: number = 0;
  const order: SORTDIR = { direction: "DESC" };
  const after: number = Date.now() - 86400000; // 1 day ago
  const before: number = Date.now();
  const username: string = "alice.smith";
  const ip: string = "198.51.100.23";
  const crudType: string = "CREATE";
  const resourceName: string = "thread";
  const targetId: string = "thread_45678";
  const target: string = "forum_12";
  const includeManagedTenants: boolean = false;

  const logs: GetAuditLogsResponse = await getAuditLogs(
    tenantId,
    limit,
    skip,
    order,
    after,
    before,
    username,
    ip,
    crudType,
    resourceName,
    targetId,
    target,
    includeManagedTenants
  );

  console.log(logs);
}

fetchAuditLogs();
[inline-code-end]

---