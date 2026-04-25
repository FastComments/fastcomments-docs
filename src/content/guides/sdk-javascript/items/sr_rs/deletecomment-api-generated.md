## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| contextUserId | string | Не |  |
| isLive | boolean | Не |  |

## Одговор

Враћа: [`DeleteComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteComment200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример за deleteComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_01";
const id: string = "comment_5f3a2b7c";
const contextUserId: string = "user_1229";
const isLive: boolean = true;
const response: DeleteComment200Response = await deleteComment(tenantId, id, contextUserId, isLive);
[inline-code-end]

---