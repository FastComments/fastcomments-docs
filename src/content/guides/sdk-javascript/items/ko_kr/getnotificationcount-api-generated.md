---
## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| userId | string | 아니요 |  |
| urlId | string | 아니요 |  |
| fromCommentId | string | 아니요 |  |
| viewed | boolean | 아니요 |  |
| type | string | 아니요 |  |

## 응답

반환: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationCount200Response.ts)

## 예제

[inline-code-attrs-start title = 'getNotificationCount 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_82a3b9f';
const userId: string = 'user_43721';
const urlId: string = 'https://news.example.com/articles/2026/06/15/coverage-123';
const fromCommentId: string = 'comment_98765';
const viewed: boolean = false;
const notificationType: string = 'mention';

const result: GetNotificationCount200Response = await getNotificationCount(tenantId, userId, urlId, fromCommentId, viewed, notificationType);
[inline-code-end]

---