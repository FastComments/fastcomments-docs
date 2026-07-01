---
## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | number | 否 |  |

## 响应

返回：[`GetTenantPackagesResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackagesResponse1.ts)

## 示例

[inline-code-attrs-start title = 'getTenantPackages 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_67890';
  const skip: number = 30;

  const packagesWithSkip: GetTenantPackagesResponse1 = await getTenantPackages(tenantId, skip);
  const packagesWithoutSkip: GetTenantPackagesResponse1 = await getTenantPackages(tenantId);

  console.log(packagesWithSkip, packagesWithoutSkip);
})();
[inline-code-end]

---