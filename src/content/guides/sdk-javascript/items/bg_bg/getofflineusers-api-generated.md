Предишни коментатори на страницата, които НЕ са в момента онлайн. Подредени по displayName.
Използвайте това след изчерпване на /users/online, за да визуализирате секция "Членове".
Курсорна пагинация по commenterName: сървърът обхожда частичния {tenantId, urlId, commenterName}
индекс от afterName напред чрез $gt, без разход за $skip.

## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| afterName | string | Не |  |
| afterUserId | string | Не |  |

## Отговор

Връща: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-9f4b2a6c';
const urlId: string = 'articles/product-launch-2025';

const offlinePageFirst: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId);

const afterName: string = 'samantha.r';
const afterUserId: string = 'user_7d3a21f9';
const offlinePageNext: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---