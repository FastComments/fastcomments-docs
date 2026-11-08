## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| limit | number | नहीं |  |
| skip | number | नहीं |  |
| order | SORTDIR | नहीं |  |
| after | number | नहीं |  |
| before | number | नहीं |  |
| username | string | नहीं |  |
| ip | string | नहीं |  |
| crudType | string | नहीं |  |
| resourceName | string | नहीं |  |
| targetId | string | नहीं |  |
| target | string | नहीं |  |
| includeManagedTenants | boolean | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetAuditLogsResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getAuditLogs उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchAuditLogs(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const limit: number = 50;
  const skip: number = 0;
  const order: SORTDIR = { direction: "DESC" };
  const after: number = Date.now() - 86400000; // 1 दिन पहले
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