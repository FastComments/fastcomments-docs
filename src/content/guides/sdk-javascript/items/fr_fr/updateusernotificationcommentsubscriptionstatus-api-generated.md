Activer ou désactiver les notifications pour un commentaire spécifique.

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| notificationId | string | Oui |  |
| optedInOrOut | UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum | Oui |  |
| commentId | string | Oui |  |
| sso | string | Non |  |

## Réponse

Retourne: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationCommentSubscriptionStatusResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de updateUserNotificationCommentSubscriptionStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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