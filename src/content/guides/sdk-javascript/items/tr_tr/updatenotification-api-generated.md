## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| updateNotificationBody | UpdateNotificationBody | Evet |  |
| userId | string | Hayır |  |

## Yanıt

Döndürür: [`UpdateNotificationResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateNotificationResponse.ts)

## Örnek

[inline-code-attrs-start title = 'updateNotification Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_5f4d2c";
const notificationId: string = "notif_9b8a7c";

const updateBody: UpdateNotificationBody = {
  enabled: false,
  sendEmail: true,
  schedule: "2023-12-01T08:00:00Z"
};

const userId: string = "user_123e4567";

const responseWithUser: UpdateNotificationResponse = await updateNotification(
  tenantId,
  notificationId,
  updateBody,
  userId
);

const responseWithoutUser: UpdateNotificationResponse = await updateNotification(
  tenantId,
  notificationId,
  updateBody
);
[inline-code-end]

---