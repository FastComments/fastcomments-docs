## Parameters

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`GetTenantPackage200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackage200Response.ts)

## 예제

[inline-code-attrs-start title = 'getTenantPackage 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_westus_01';
const packageId: string = 'pkg_premium_annual_2026';
interface FetchOptions { includeArchived?: boolean }
const options: FetchOptions = { includeArchived: false };
const result: GetTenantPackage200Response = await getTenantPackage(tenantId, packageId);
[inline-code-end]

---