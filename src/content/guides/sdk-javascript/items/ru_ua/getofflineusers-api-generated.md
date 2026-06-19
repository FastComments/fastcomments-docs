Прошлые комментаторы на странице, которые НЕ находятся в сети. Отсортировано по displayName.
Используйте это после исчерпания /users/online, чтобы отобразить раздел "Участники".
Постраничная пагинация курсором по commenterName: сервер проходит по частичному {tenantId, urlId, commenterName} индексу от afterName вперёд через $gt, без стоимости $skip.

## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Ответ

Возвращает: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-9f4b2a6c';
const urlId: string = 'articles/product-launch-2025';

const offlinePageFirst: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId);

const afterName: string = 'samantha.r';
const afterUserId: string = 'user_7d3a21f9';
const offlinePageNext: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---