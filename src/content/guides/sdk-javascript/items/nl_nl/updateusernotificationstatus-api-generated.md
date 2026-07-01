## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| notificationId | string | Ja |  |
| newStatus | UpdateUserNotificationStatusNewStatusEnum | Ja |  |
| sso | string | Nee |  |

## Response

Retourneert: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatusResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'updateUserNotificationStatus Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp";
const notificationId: string = "notif-20231101-001";
const newStatus: UpdateUserNotificationStatusNewStatusEnum = UpdateUserNotificationStatusNewStatusEnum.Dismissed;
const ssoToken: string = "sso-9f8e7d6c5b4a";

const result: UpdateUserNotificationStatusResponse = await updateUserNotificationStatus(
  tenantId,
  notificationId,
  newStatus,
  ssoToken
);
[inline-code-end]