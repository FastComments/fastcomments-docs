---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updateNotificationBody | UpdateNotificationBody | 예 |  |
| userId | string | 아니요 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 예제

[inline-code-attrs-start title = 'updateNotification 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'd6f9b2a4-1b2c-4e5f-9a7b-3c2d1e4f5a6b';
const id: string = 'notification-78f1c3e4';
const updateNotificationBody: UpdateNotificationBody = {} as UpdateNotificationBody;
const userId: string = 'user-9b3f2a1c';

const responseWithUser: APIEmptyResponse = await updateNotification(tenantId, id, updateNotificationBody, userId);
const responseWithoutUser: APIEmptyResponse = await updateNotification(tenantId, id, updateNotificationBody);
[inline-code-end]

---