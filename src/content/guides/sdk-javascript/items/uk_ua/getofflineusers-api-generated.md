Колишні коментатори на сторінці, які зараз не в мережі. Відсортовано за displayName.
Use this after exhausting /users/online to render a "Учасники" section.
Курсорна пагінація за commenterName: сервер проходить частковий індекс {tenantId, urlId, commenterName}
index from afterName forward via $gt, no $skip cost.

## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| urlId | string | Так |  |
| afterName | string | Ні |  |
| afterUserId | string | Ні |  |

## Відповідь

Повертає: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад використання getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-9f4b2a6c';
const urlId: string = 'articles/product-launch-2025';

const offlinePageFirst: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId);

const afterName: string = 'samantha.r';
const afterUserId: string = 'user_7d3a21f9';
const offlinePageNext: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---