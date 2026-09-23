## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| notificationId | string | Da |  |
| newStatus | UpdateUserNotificationStatusNewStatusEnum | Da |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatusResponse.ts)

## Primer

[inline-code-attrs-start title = 'updateUserNotificationStatus Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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