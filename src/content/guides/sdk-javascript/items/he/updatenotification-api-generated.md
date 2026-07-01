## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |
| updateNotificationBody | UpdateNotificationBody | כן |  |
| userId | string | לא |  |

## תגובה

מחזירה: [`UpdateNotificationResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateNotificationResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת updateNotification'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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