## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| userId | string | Ні |  |
| urlId | string | Ні |  |
| fromCommentId | string | Ні |  |
| viewed | boolean | Ні |  |
| type | string | Ні |  |

## Відповідь

Повертає: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationCount200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_abc123';
const userId: string = 'user_987654321';
const urlId: string = 'https://example.com/news/2026/new-features';
const viewed: boolean = false;
const type: string = 'reply';
const notificationCountResponse: GetNotificationCount200Response = await getNotificationCount(tenantId, userId, urlId, undefined, viewed, type);
[inline-code-end]

---