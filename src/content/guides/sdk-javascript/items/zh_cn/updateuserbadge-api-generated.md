## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateUserBadgeParams | UpdateUserBadgeParams | 是 |  |

## 响应

返回: [`UpdateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserBadge200Response.ts)

## 示例

[inline-code-attrs-start title = 'updateUserBadge 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a8b7c";
const id: string = "badge_a1b2c3";
const updateUserBadgeParams: UpdateUserBadgeParams = {
  title: "Community Helper",
  description: "Awarded for providing 50 helpful answers",
  iconUrl: "https://cdn.fastcomments.com/badges/community-helper.png",
  isActive: true,
  expiryDate: undefined
};
const result: UpdateUserBadge200Response = await updateUserBadge(tenantId, id, updateUserBadgeParams);
[inline-code-end]

---