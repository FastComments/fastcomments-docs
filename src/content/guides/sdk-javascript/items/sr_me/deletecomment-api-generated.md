## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| contextUserId | string | Не |  |
| isLive | boolean | Не |  |

## Одговор

Враћа: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentResult.ts)

## Пример

[inline-code-attrs-start title = 'Пример deleteComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-834';
const id: string = 'cmt_9f3b2d7a';
const contextUserId: string = 'user_4b2f6c88-1a2b-4c3d-9e5f-123456789abc';
const isLive: boolean = true;
const result: DeleteCommentResult = await deleteComment(tenantId, id, contextUserId, isLive);
[inline-code-end]

---