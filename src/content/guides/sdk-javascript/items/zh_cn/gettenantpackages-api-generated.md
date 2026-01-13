## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | number | 否 |  |

## 响应

返回: [`GetTenantPackages200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackages200Response.ts)

## 示例

[inline-code-attrs-start title = 'getTenantPackages 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4f3a9c2d';
const skip: number = 25;
const packagesResponse: GetTenantPackages200Response = await getTenantPackages(tenantId);
const pagedPackagesResponse: GetTenantPackages200Response = await getTenantPackages(tenantId, skip);
[inline-code-end]

---