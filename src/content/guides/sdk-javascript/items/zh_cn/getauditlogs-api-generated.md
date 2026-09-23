## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| limit | number | 否 |  |
| skip | number | 否 |  |
| order | SORTDIR | 否 |  |
| after | number | 否 |  |
| before | number | 否 |  |
| username | string | 否 |  |
| ip | string | 否 |  |
| crudType | string | 否 |  |
| resourceName | string | 否 |  |
| targetId | string | 否 |  |
| target | string | 否 |  |
| includeManagedTenants | boolean | 否 |  |

## 响应

返回：[`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetAuditLogsResponse.ts)

## 示例

[inline-code-attrs-start title = 'getAuditLogs 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchAuditLogs(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const limit: number = 50;
  const skip: number = 0;
  const order: SORTDIR = { direction: "DESC" };
  const after: number = Date.now() - 86400000; // 1 天前
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