## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Відповідь

Повертає: [`DeleteNotificationCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteNotificationCountResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run() {
  const tenantId: string = "tenant_12345";
  const notificationId: string = "notif_98765";
  const result: DeleteNotificationCountResponse = await deleteNotificationCount(tenantId, notificationId);
  console.log(result);
}
run();
[inline-code-end]

---