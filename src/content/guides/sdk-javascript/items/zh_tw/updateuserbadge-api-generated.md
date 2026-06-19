## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateUserBadgeParams | UpdateUserBadgeParams | 是 |  |

## 回應

回傳: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptySuccessResponse.ts)

## 範例

[inline-code-attrs-start title = 'updateUserBadge 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-web-tenant-7";
const id: string = "badge_48f2a9";
const updateUserBadgeParams: UpdateUserBadgeParams = {
  label: "Community Champion",
  description: "Awarded for exceptional moderation and sustained helpful responses",
  active: true,
  expiresAt: "2026-12-31T23:59:59Z", // 示範可選的到期時間
  notifyUsers: true,
  metadata: { awardedBy: "moderator_jane" }
};
const result: APIEmptySuccessResponse = await updateUserBadge(tenantId, id, updateUserBadgeParams);
[inline-code-end]

---