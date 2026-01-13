## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| userId | string | Не |  |
| urlId | string | Не |  |
| fromCommentId | string | Не |  |
| viewed | boolean | Не |  |
| type | string | Не |  |

## Отговор

Връща: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationCount200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8a9b7c';
const userId: string = 'user_42b3c';
const urlId: string = 'https://blog.example.com/posts/introducing-new-editor';
const fromCommentId: string | undefined = undefined;
const viewed: boolean = false;
const type: string = 'mention';
const result: GetNotificationCount200Response = await getNotificationCount(tenantId, userId, urlId, fromCommentId, viewed, type);
[inline-code-end]

---