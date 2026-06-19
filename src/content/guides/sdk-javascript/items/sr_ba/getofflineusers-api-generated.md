Бивши коментатори на страници који тренутно НИСУ онлајн. Сортирано по displayName.
Користите ово након што исцрпите /users/online да прикажете одјељак "Members".
Курсор пагинација по commenterName: сервер пролази парцијални {tenantId, urlId, commenterName}
индекс од afterName напред користећи $gt, без трошка $skip.

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| afterName | string | Не |  |
| afterUserId | string | Не |  |

## Одговор

Враћа: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример употребе getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-9f4b2a6c';
const urlId: string = 'articles/product-launch-2025';

const offlinePageFirst: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId);

const afterName: string = 'samantha.r';
const afterUserId: string = 'user_7d3a21f9';
const offlinePageNext: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---