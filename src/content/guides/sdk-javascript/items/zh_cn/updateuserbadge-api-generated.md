## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateUserBadgeParams | UpdateUserBadgeParams | 是 |  |

## 响应

返回：[`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptySuccessResponse.ts)

## 示例

[inline-code-attrs-start title = 'updateUserBadge 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-web-tenant-7";
const id: string = "badge_48f2a9";
const updateUserBadgeParams: UpdateUserBadgeParams = {
  label: "Community Champion",
  description: "Awarded for exceptional moderation and sustained helpful responses",
  active: true,
  expiresAt: "2026-12-31T23:59:59Z", // 可选的到期时间示例
  notifyUsers: true,
  metadata: { awardedBy: "moderator_jane" }
};
const result: APIEmptySuccessResponse = await updateUserBadge(tenantId, id, updateUserBadgeParams);
[inline-code-end]

---