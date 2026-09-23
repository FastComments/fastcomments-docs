## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|--------------|
| tenantId | string | Ja |  |
| notificationId | string | Ja |  |
| newStatus | UpdateUserNotificationStatusNewStatusEnum | Ja |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatusResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'updateUserNotificationStatus Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const notificationId: string = "notif_9876";
const newStatus: UpdateUserNotificationStatusNewStatusEnum = UpdateUserNotificationStatusNewStatusEnum.READ;
const ssoToken: string = "sso_token_abc";

const responseWithSso: UpdateUserNotificationStatusResponse = await updateUserNotificationStatus(
  tenantId,
  notificationId,
  newStatus,
  ssoToken
);

const responseWithoutSso: UpdateUserNotificationStatusResponse = await updateUserNotificationStatus(
  tenantId,
  notificationId,
  newStatus
);
[inline-code-end]