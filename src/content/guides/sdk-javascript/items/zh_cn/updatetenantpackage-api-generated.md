---
## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateTenantPackageBody | UpdateTenantPackageBody | 是 |  |

## 响应

返回: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 示例

[inline-code-attrs-start title = 'updateTenantPackage 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_sf_001";
  const id: string = "pkg-premium-v2";
  const updateTenantPackageBody: UpdateTenantPackageBody = {
    name: "San Francisco Premium",
    enabled: true,
    customConfig: { maxComments: 500 },
    tosConfig: { required: true } // 示例展示了可选字段；其他字段已省略
  } as UpdateTenantPackageBody;
  const result: FlagCommentPublic200Response = await updateTenantPackage(tenantId, id, updateTenantPackageBody);
  console.log(result);
})();
[inline-code-end]

---