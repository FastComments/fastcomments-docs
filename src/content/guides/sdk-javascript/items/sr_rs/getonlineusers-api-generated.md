Тренутно онлајн гледаоци странице: људи чија websocket сесија је тренутно претплаћена на страницу.
Враћа anonCount + totalCount (претплатнике у целој соби, укључујући анонимне гледаоце које не набрајамо).

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| afterName | string | Не |  |
| afterUserId | string | Не |  |

## Одговор

Враћа: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOnlineResponse.ts)

## Пример

[inline-code-attrs-start title = 'getOnlineUsers Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3c2b7';
const urlId: string = 'article-2026-06-19-site-update';
const afterName: string = 'michael.hansen';
const afterUserId: string = 'user_00421';
const onlineUsers: PageUsersOnlineResponse = await getOnlineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---