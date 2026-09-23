Enable or disable notifications for a specific comment.

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| notificationId | string | Ναι |  |
| optedInOrOut | UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum | Ναι |  |
| commentId | string | Ναι |  |
| sso | string | Όχι |  |

## Απάντηση

Επιστρέφει: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationCommentSubscriptionStatusResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα updateUserNotificationCommentSubscriptionStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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