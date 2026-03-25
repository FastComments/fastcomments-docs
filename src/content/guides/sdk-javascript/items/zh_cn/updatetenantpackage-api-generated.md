## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateTenantPackageBody | UpdateTenantPackageBody | 是 |  |

## 响应

返回：[`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 示例

[inline-code-attrs-start title = 'updateTenantPackage 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_3b7f9d-prod";
const id: string = "pkg_enterprise_2026";
const updateTenantPackageBody: UpdateTenantPackageBody = {
  name: "Enterprise Plus",
  isActive: true,
  // 有意省略可选字段 (例如, description, limits)
} as UpdateTenantPackageBody;
const result: FlagCommentPublic200Response = await updateTenantPackage(tenantId, id, updateTenantPackageBody);
[inline-code-end]

---