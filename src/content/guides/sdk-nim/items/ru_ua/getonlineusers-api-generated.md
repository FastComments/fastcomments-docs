Текущие онлайн‑просмотрщики страницы: люди, чья websocket‑сессия сейчас подписана на страницу.  
Возвращает anonCount + totalCount (подписчики по всей комнате, включая анонимных зрителей, которых мы не перечисляем).

## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | GetOnlineUsersOptions | No |  |

## Ответ

Возвращает: [`Option[PageUsersOnlineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_online_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getOnlineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetOnlineUsersOptions()
let (onlineUsersOpt, httpResp) = client.getOnlineUsers(tenantId = "my-tenant-123", urlId = "news/article-title", options = opts)
if onlineUsersOpt.isSome:
  let onlineUsers = onlineUsersOpt.get()
  echo onlineUsers
[inline-code-end]