## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |
| contextUserId | string | Ні |  |
| isLive | boolean | Ні |  |

## Відповідь

Повертає: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentResult.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад використання deleteComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-834';
const id: string = 'cmt_9f3b2d7a';
const contextUserId: string = 'user_4b2f6c88-1a2b-4c3d-9e5f-123456789abc';
const isLive: boolean = true;
const result: DeleteCommentResult = await deleteComment(tenantId, id, contextUserId, isLive);
[inline-code-end]

---