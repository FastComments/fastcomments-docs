---
Претходни коментатори на страници који НИСУ тренутно на мрежи. Сортирано по displayName.
Користите ово након исцрпљивања /users/online да бисте приказали секцију "Чланови".
Курсорска пагинација по commenterName: сервер пролази делимичан индекс {tenantId, urlId, commenterName}
индекс од afterName унапред помоћу $gt, без трошкова $skip.

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| afterName | string | Не |  |
| afterUserId | string | Не |  |

## Одговор

Враћа: [`GetOfflineUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsers200Response.ts)

## Пример

[inline-code-attrs-start title = 'getOfflineUsers Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_prod_001';
const urlId: string = 'article-2026-06-15-how-ai-impacts';
const afterName: string = 'michael.smith';
const afterUserId: string = 'user_72b9';

const response: GetOfflineUsers200Response = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---