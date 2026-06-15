---
Прошли коментатори на страници који тренутно НИСУ онлајн. Сортирано по displayName.
Користите ово након што исцрпите /users/online да бисте приказали одељак "Чланови".
Курсорска пагинација по commenterName: сервер пролази делимични индекс {tenantId, urlId, commenterName} почевши од afterName унапред користећи $gt, без трошкова $skip.

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Одговор

Враћа: [`GetOfflineUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsers200Response.ts)

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