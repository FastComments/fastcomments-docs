---
## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| updateNotificationBody | UpdateNotificationBody | Da |  |
| userId | string | Ne |  |

## Odgovor

Vrne: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Primer

[inline-code-attrs-start title = 'updateNotification Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'd6f9b2a4-1b2c-4e5f-9a7b-3c2d1e4f5a6b';
const id: string = 'notification-78f1c3e4';
const updateNotificationBody: UpdateNotificationBody = {} as UpdateNotificationBody;
const userId: string = 'user-9b3f2a1c';

const responseWithUser: APIEmptyResponse = await updateNotification(tenantId, id, updateNotificationBody, userId);
const responseWithoutUser: APIEmptyResponse = await updateNotification(tenantId, id, updateNotificationBody);
[inline-code-end]

---