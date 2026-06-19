## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateNotificationBody | UpdateNotificationBody | Ja |  |
| userId | string | Nein |  |

## Antwort

Gibt zurück: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für updateNotification'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'd6f9b2a4-1b2c-4e5f-9a7b-3c2d1e4f5a6b';
const id: string = 'notification-78f1c3e4';
const updateNotificationBody: UpdateNotificationBody = {} as UpdateNotificationBody;
const userId: string = 'user-9b3f2a1c';

const responseWithUser: APIEmptyResponse = await updateNotification(tenantId, id, updateNotificationBody, userId);
const responseWithoutUser: APIEmptyResponse = await updateNotification(tenantId, id, updateNotificationBody);
[inline-code-end]

---