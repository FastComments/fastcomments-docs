Бывшие комментаторы на странице, которые В НАСТОЯЩЕЕ ВРЕМЯ НЕ В сети. Отсортировано по displayName.
Используйте это после исчерпания /users/online для отображения раздела «Участники».
Пагинация курсором по commenterName: сервер проходит по частичному индексу {tenantId, urlId, commenterName} от afterName вперёд с помощью $gt, без стоимости $skip.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Response

Returns: [`GetOfflineUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsers200Response.ts)

## Example

[inline-code-attrs-start title = 'Пример getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_prod_001';
const urlId: string = 'article-2026-06-15-how-ai-impacts';
const afterName: string = 'michael.smith';
const afterUserId: string = 'user_72b9';

const response: GetOfflineUsers200Response = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---