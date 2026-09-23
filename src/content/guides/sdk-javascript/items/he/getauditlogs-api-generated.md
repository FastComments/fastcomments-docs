## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| limit | number | לא |  |
| skip | number | לא |  |
| order | SORTDIR | לא |  |
| after | number | לא |  |
| before | number | לא |  |
| username | string | לא |  |
| ip | string | לא |  |
| crudType | string | לא |  |
| resourceName | string | לא |  |
| targetId | string | לא |  |
| target | string | לא |  |
| includeManagedTenants | boolean | לא |  |

## תגובה

מחזיר: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetAuditLogsResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getAuditLogs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchAuditLogs(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const limit: number = 50;
  const skip: number = 0;
  const order: SORTDIR = { direction: "DESC" };
  const after: number = Date.now() - 86400000; // לפני יום אחד
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