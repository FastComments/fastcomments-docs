## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |

## תגובה

מחזיר: [`GetDomainConfigsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetDomainConfigsResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getDomainConfigs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9b7e4f3c";
const domainConfigs: GetDomainConfigsResponse = await getDomainConfigs(tenantId);

async function fetchDomainConfigs(tenant: string, useCache?: boolean): Promise<GetDomainConfigsResponse> {
  if (useCache) return domainConfigs;
  const fresh: GetDomainConfigsResponse = await getDomainConfigs(tenant);
  return fresh;
}

const freshConfigs: GetDomainConfigsResponse = await fetchDomainConfigs(tenantId, false);
[inline-code-end]

---