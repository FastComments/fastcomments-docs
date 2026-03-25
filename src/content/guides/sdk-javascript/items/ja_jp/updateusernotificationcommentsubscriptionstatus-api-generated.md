特定のコメントに対する通知を有効化または無効化します。

## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| notificationId | string | はい |  |
| optedInOrOut | UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum | はい |  |
| commentId | string | はい |  |
| sso | string | いいえ |  |

## レスポンス

返却値: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

## 例

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-001';
const notificationId: string = 'notif-2026-03-25-01';
const commentId: string = 'cmt-8f3a2b';
const optedInOrOut: UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum = UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum.OptIn;
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.sso-payload.signature';
const result: UpdateUserNotificationStatus200Response = await updateUserNotificationCommentSubscriptionStatus(tenantId, notificationId, optedInOrOut, commentId, sso);
[inline-code-end]

---