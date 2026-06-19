## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| userId | string | 아니오 |  |
| urlId | string | 아니오 |  |
| fromCommentId | string | 아니오 |  |
| viewed | boolean | 아니오 |  |
| type | string | 아니오 |  |
| skip | number | 아니오 |  |

## 응답

반환값: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationsResponse.ts)

## 예제

[inline-code-attrs-start title = 'getNotifications 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_98b3f';
const userId: string = 'user_8a3f';
const urlId: string = '/blog/2026/new-feature';
const viewed: boolean = false;
const type: string = 'reply';
const skip: number = 10;
const notifications: GetNotificationsResponse = await getNotifications(tenantId, userId, urlId, undefined, viewed, type, skip);
[inline-code-end]