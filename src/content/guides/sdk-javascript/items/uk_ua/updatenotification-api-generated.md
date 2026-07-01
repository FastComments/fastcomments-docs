## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |
| updateNotificationBody | UpdateNotificationBody | Так |  |
| userId | string | Ні |  |

## Відповідь

Повертає: [`UpdateNotificationResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateNotificationResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад updateNotification'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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