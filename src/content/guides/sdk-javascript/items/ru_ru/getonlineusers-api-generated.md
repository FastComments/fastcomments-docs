Текущие онлайн-зрители страницы: люди, чья websocket-сессия в данный момент подписана на эту страницу.
Возвращает anonCount + totalCount (подписчики комнаты в целом, включая анонимных зрителей, которых мы не перечисляем).

## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| afterName | string | Нет |  |
| afterUserId | string | Нет |  |

## Ответ

Возвращает: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOnlineResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример использования getOnlineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3c2b7';
const urlId: string = 'article-2026-06-19-site-update';
const afterName: string = 'michael.hansen';
const afterUserId: string = 'user_00421';
const onlineUsers: PageUsersOnlineResponse = await getOnlineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---