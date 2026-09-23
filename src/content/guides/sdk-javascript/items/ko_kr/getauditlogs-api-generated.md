## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | 예 |  |
| limit | number | 아니오 |  |
| skip | number | 아니오 |  |
| order | SORTDIR | 아니오 |  |
| after | number | 아니오 |  |
| before | number | 아니오 |  |
| username | string | 아니오 |  |
| ip | string | 아니오 |  |
| crudType | string | 아니오 |  |
| resourceName | string | 아니오 |  |
| targetId | string | 아니오 |  |
| target | string | 아니오 |  |
| includeManagedTenants | boolean | 아니오 |  |

## 응답

반환: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetAuditLogsResponse.ts)

## 예시

[inline-code-attrs-start title = 'getAuditLogs 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchAuditLogs(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const limit: number = 50;
  const skip: number = 0;
  const order: SORTDIR = { direction: "DESC" };
  const after: number = Date.now() - 86400000; // 1일 전
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