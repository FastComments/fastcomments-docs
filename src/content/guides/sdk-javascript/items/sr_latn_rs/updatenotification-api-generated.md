## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| id | string | Da |  |
| updateNotificationBody | UpdateNotificationBody | Da |  |
| userId | string | Ne |  |

## Odgovor

Vraća: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer updateNotification'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function main(): Promise<void> {
  const tenantId: string = "tenant-abc123";
  const notificationId: string = "notif-9876";
  const updateBody: UpdateNotificationBody = {
    title: "System Maintenance",
    message: "Scheduled maintenance at 02:00 UTC.",
    enabled: true
  };
  const userId: string = "user-42";

  const resultWithoutUser: APIEmptyResponse = await updateNotification(tenantId, notificationId, updateBody);
  const resultWithUser: APIEmptyResponse = await updateNotification(tenantId, notificationId, updateBody, userId);
}
[inline-code-end]