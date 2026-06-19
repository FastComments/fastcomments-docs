## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | 是 |  |

## 响应

返回：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 示例

[inline-code-attrs-start title = 'replaceTenantPackage 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3b1c';
const id: string = 'pkg_pro_2026';
const replaceTenantPackageBody: ReplaceTenantPackageBody = {
  planCode: 'pro_annual',
  seats: 12,
  expiresAt: '2027-01-01T00:00:00Z',
  autoRenew: true, // 可选标志，演示可选参数
  notes: 'Upgrade for team collaboration'
};
const result: APIEmptyResponse = await replaceTenantPackage(tenantId, id, replaceTenantPackageBody);
[inline-code-end]

---