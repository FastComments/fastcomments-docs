## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| notificationId | string | Sì |  |
| newStatus | UpdateUserNotificationStatusNewStatusEnum | Sì |  |
| sso | string | No |  |

## Risposta

Restituisce: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatusResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di updateUserNotificationStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-8d3f2b7c';
const notificationId: string = 'notification-587a2b9f';
const newStatus: UpdateUserNotificationStatusNewStatusEnum = UpdateUserNotificationStatusNewStatusEnum.Read;
const sso: string = 'sso-token-1a2b3c4d5e6f';
const result: UpdateUserNotificationStatusResponse = await updateUserNotificationStatus(tenantId, notificationId, newStatus, sso);
[inline-code-end]

---