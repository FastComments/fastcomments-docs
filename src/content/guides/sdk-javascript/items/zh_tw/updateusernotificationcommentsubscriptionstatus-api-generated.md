啟用或停用對特定評論的通知。

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| notificationId | string | 是 |  |
| optedInOrOut | UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum | 是 |  |
| commentId | string | 是 |  |
| sso | string | 否 |  |

## 回應

回傳: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

## 範例

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-001';
const notificationId: string = 'notif-2026-03-25-01';
const commentId: string = 'cmt-8f3a2b';
const optedInOrOut: UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum = UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum.OptIn;
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.sso-payload.signature';
const result: UpdateUserNotificationStatus200Response = await updateUserNotificationCommentSubscriptionStatus(tenantId, notificationId, optedInOrOut, commentId, sso);
[inline-code-end]

---