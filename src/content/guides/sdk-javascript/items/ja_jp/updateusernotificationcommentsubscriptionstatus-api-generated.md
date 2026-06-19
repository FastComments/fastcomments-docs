---
特定のコメントの通知を有効または無効にします。

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| notificationId | string | はい |  |
| optedInOrOut | UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum | はい |  |
| commentId | string | はい |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationCommentSubscriptionStatusResponse.ts)

## 例

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_92a1c4';
const notificationId: string = 'notif_3f7b2e9a-4d1b-11ec-8d3d-0242ac130003';
const optedInOrOut: UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum =
  UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum.OPTED_IN;
const commentId: string = 'cmt_8f4b2e7a';
const sso: string | undefined = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fake.payload';
const response: UpdateUserNotificationCommentSubscriptionStatusResponse =
  await updateUserNotificationCommentSubscriptionStatus(tenantId, notificationId, optedInOrOut, commentId, sso);
[inline-code-end]

---