## Параметри

| Назва | Тип | Обов'язково | Опис |
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
const tenantId: string = 'tenant_82a3b9f';
const userId: string = 'user_43721';
const urlId: string = 'https://news.example.com/articles/2026/06/15/coverage-123';
const fromCommentId: string = 'comment_98765';
const viewed: boolean = false;
const notificationType: string = 'mention';

const result: GetNotificationCount200Response = await getNotificationCount(tenantId, userId, urlId, fromCommentId, viewed, notificationType);
[inline-code-end]

---