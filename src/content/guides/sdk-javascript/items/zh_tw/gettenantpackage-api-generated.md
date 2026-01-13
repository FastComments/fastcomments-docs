## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

回傳: [`GetTenantPackage200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackage200Response.ts)

## 範例

[inline-code-attrs-start title = 'getTenantPackage 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_westus_01';
const packageId: string = 'pkg_premium_annual_2026';
interface FetchOptions { includeArchived?: boolean }
const options: FetchOptions = { includeArchived: false };
const result: GetTenantPackage200Response = await getTenantPackage(tenantId, packageId);
[inline-code-end]

---