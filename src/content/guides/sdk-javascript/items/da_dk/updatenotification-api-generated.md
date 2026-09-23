## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateNotificationBody | UpdateNotificationBody | Ja |  |
| userId | string | Nej |  |

## Svar

Returnerer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'updateNotification Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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