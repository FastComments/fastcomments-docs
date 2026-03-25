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
const tenantId: string = 'tenant_7f3b2c8';
const packageId: string = 'pkg_standard_2026';
const requestOptions: { includeConfig?: boolean } = { includeConfig: true };
const packageResponse: GetTenantPackage200Response = await getTenantPackage(tenantId, packageId);
[inline-code-end]

---