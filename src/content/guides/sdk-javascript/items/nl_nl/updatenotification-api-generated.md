## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateNotificationBody | UpdateNotificationBody | Ja |  |
| userId | string | Nee |  |

## Respons

Retourneert: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld updateNotification'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'd6f9b2a4-1b2c-4e5f-9a7b-3c2d1e4f5a6b';
const id: string = 'notification-78f1c3e4';
const updateNotificationBody: UpdateNotificationBody = {} as UpdateNotificationBody;
const userId: string = 'user-9b3f2a1c';

const responseWithUser: APIEmptyResponse = await updateNotification(tenantId, id, updateNotificationBody, userId);
const responseWithoutUser: APIEmptyResponse = await updateNotification(tenantId, id, updateNotificationBody);
[inline-code-end]

---