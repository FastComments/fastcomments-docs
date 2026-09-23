Benachrichtigungen für einen bestimmten Kommentar aktivieren oder deaktivieren.

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| notificationId | string | Ja |  |
| optedInOrOut | UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum | Ja |  |
| commentId | string | Ja |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationCommentSubscriptionStatusResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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