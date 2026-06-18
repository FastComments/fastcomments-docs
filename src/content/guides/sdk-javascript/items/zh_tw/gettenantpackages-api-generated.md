## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | number | 否 |  |

## 回應

回傳: [`GetTenantPackages200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackages200Response.ts)

## 範例

[inline-code-attrs-start title = 'getTenantPackages 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8421';
const packagesWithSkip: GetTenantPackages200Response = await getTenantPackages(tenantId, 25);
const packagesWithoutSkip: GetTenantPackages200Response = await getTenantPackages(tenantId);
[inline-code-end]