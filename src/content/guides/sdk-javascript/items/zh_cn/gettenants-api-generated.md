## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| meta | string | No |  |
| skip | number | No |  |

## 响应

返回：[`GetTenantsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantsResponse1.ts)

## 示例

[inline-code-attrs-start title = 'getTenants 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-9f8e7d6c";
  const resultOnlyId: GetTenantsResponse1 = await getTenants(tenantId);
  const resultWithMeta: GetTenantsResponse1 = await getTenants(tenantId, "full");
  const resultAllParams: GetTenantsResponse1 = await getTenants(tenantId, "full", 15);
  console.log(resultOnlyId, resultWithMeta, resultAllParams);
})();
[inline-code-end]