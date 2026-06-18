## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| notificationId | string | Da |  |
| newStatus | UpdateUserNotificationStatusNewStatusEnum | Da |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

## Primjer

[inline-code-attrs-start title = 'updateUserNotificationStatus Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_5f2a8b9c';
const notificationId: string = 'notif_987654321';
const newStatus: UpdateUserNotificationStatusNewStatusEnum = UpdateUserNotificationStatusNewStatusEnum.Read;
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.payload.signature';
const result: UpdateUserNotificationStatus200Response = await updateUserNotificationStatus(tenantId, notificationId, newStatus, sso);
[inline-code-end]

---