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
const tenantId: string = 'tenant-7b3c2f';
const skipCount: number = 10;
const packages: GetTenantPackages200Response = await getTenantPackages(tenantId, skipCount);
const packagesFromStart: GetTenantPackages200Response = await getTenantPackages(tenantId);
[inline-code-end]

---