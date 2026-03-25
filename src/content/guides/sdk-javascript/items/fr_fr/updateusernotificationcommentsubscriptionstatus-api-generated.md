Activer ou désactiver les notifications pour un commentaire spécifique.

## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| notificationId | string | Oui |  |
| optedInOrOut | UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum | Oui |  |
| commentId | string | Oui |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de updateUserNotificationCommentSubscriptionStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-001';
const notificationId: string = 'notif-2026-03-25-01';
const commentId: string = 'cmt-8f3a2b';
const optedInOrOut: UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum = UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum.OptIn;
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.sso-payload.signature';
const result: UpdateUserNotificationStatus200Response = await updateUserNotificationCommentSubscriptionStatus(tenantId, notificationId, optedInOrOut, commentId, sso);
[inline-code-end]