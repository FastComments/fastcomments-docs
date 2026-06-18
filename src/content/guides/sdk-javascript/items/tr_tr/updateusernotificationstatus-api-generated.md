## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| notificationId | string | Evet |  |
| newStatus | UpdateUserNotificationStatusNewStatusEnum | Evet |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

## Örnek

[inline-code-attrs-start title = 'updateUserNotificationStatus Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_5f2a8b9c';
const notificationId: string = 'notif_987654321';
const newStatus: UpdateUserNotificationStatusNewStatusEnum = UpdateUserNotificationStatusNewStatusEnum.Read;
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.payload.signature';
const result: UpdateUserNotificationStatus200Response = await updateUserNotificationStatus(tenantId, notificationId, newStatus, sso);
[inline-code-end]

---