## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | 是 |  |

## 响应

返回： [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 示例

[inline-code-attrs-start title = 'replaceTenantPackage 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-9b72f2';
const packageId: string = 'pkg-prod-v2';
const replaceTenantPackageBody: ReplaceTenantPackageBody = {
  name: 'Premium Moderation Bundle',
  enabled: true,
  maxModerators: 4,
  // 类似 "notes" 或 "trialExpiry" 的可选字段在此处故意省略
} as ReplaceTenantPackageBody;
const result: FlagCommentPublic200Response = await replaceTenantPackage(
  tenantId,
  packageId,
  replaceTenantPackageBody
);
[inline-code-end]

---