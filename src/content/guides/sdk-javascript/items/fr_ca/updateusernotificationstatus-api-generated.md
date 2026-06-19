## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| notificationId | string | Oui |  |
| newStatus | UpdateUserNotificationStatusNewStatusEnum | Oui |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatusResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de updateUserNotificationStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-8d3f2b7c';
const notificationId: string = 'notification-587a2b9f';
const newStatus: UpdateUserNotificationStatusNewStatusEnum = UpdateUserNotificationStatusNewStatusEnum.Read;
const sso: string = 'sso-token-1a2b3c4d5e6f';
const result: UpdateUserNotificationStatusResponse = await updateUserNotificationStatus(tenantId, notificationId, newStatus, sso);
[inline-code-end]

---