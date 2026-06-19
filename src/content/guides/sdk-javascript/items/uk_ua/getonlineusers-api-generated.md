Користувачі, які зараз онлайн на сторінці: люди, чиї websocket-сесії підписані на цю сторінку в цей момент.
Повертає anonCount + totalCount (підписники кімнати в цілому, включно з анонімними глядачами, яких ми не перераховуємо).

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| urlId | string | Так |  |
| afterName | string | Ні |  |
| afterUserId | string | Ні |  |

## Відповідь

Повертає: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOnlineResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getOnlineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3c2b7';
const urlId: string = 'article-2026-06-19-site-update';
const afterName: string = 'michael.hansen';
const afterUserId: string = 'user_00421';
const onlineUsers: PageUsersOnlineResponse = await getOnlineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---