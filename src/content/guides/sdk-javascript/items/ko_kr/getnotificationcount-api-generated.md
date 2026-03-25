## 매개변수

| Name | 유형 | 필수 | 설명 |
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
const tenantId: string = 'tenant_abc123';
const userId: string = 'user_987654321';
const urlId: string = 'https://example.com/news/2026/new-features';
const viewed: boolean = false;
const type: string = 'reply';
const notificationCountResponse: GetNotificationCount200Response = await getNotificationCount(tenantId, userId, urlId, undefined, viewed, type);
[inline-code-end]

---