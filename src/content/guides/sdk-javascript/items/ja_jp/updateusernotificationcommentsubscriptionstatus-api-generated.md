特定のコメントに対する通知を有効化または無効化します。

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| notificationId | string | はい |  |
| optedInOrOut | UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum | はい |  |
| commentId | string | はい |  |
| sso | string | いいえ |  |

## レスポンス

返却: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationCommentSubscriptionStatusResponse.ts)

## 例

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const notificationId: string = "notif_9876";
  const commentId: string = "comment_abc123";
  const optedInOrOut: UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum =
    UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum.OptIn;
  const sso: string = "sso_token_456";

  const resultWithSso: UpdateUserNotificationCommentSubscriptionStatusResponse = await updateUserNotificationCommentSubscriptionStatus(
    tenantId,
    notificationId,
    optedInOrOut,
    commentId,
    sso
  );

  const resultWithoutSso: UpdateUserNotificationCommentSubscriptionStatusResponse = await updateUserNotificationCommentSubscriptionStatus(
    tenantId,
    notificationId,
    optedInOrOut,
    commentId
  );
}
[inline-code-end]

---