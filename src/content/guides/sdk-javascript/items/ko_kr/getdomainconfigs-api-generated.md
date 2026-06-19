## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |

## 응답

반환: [`GetDomainConfigsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetDomainConfigsResponse.ts)

## 예제

[inline-code-attrs-start title = 'getDomainConfigs 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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