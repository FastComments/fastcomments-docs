Предыдущие комментаторы на странице, которые в настоящий момент НЕ в сети. Отсортировано по displayName.
Используйте это после исчерпания /users/online для отображения раздела «Участники».
Постраничная пагинация (cursor) по commenterName: сервер проходит по частичному {tenantId, urlId, commenterName}
индексу от afterName вперёд через $gt, без стоимости $skip.

## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| afterName | string | Нет |  |
| afterUserId | string | Нет |  |

## Ответ

Возвращает: [`GetOfflineUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsers200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_prod_001';
const urlId: string = 'article-2026-06-15-how-ai-impacts';
const afterName: string = 'michael.smith';
const afterUserId: string = 'user_72b9';

const response: GetOfflineUsers200Response = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---