Текущие онлайн‑просмотры страницы: люди, чья веб‑сокет сессия подписана на страницу прямо сейчас.  
Возвращает anonCount + totalCount (подписчики комнаты, включая анонимных зрителей, которых мы не перечисляем).

## Parameters

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| options | GetOnlineUsersOptions | Нет |  |

## Response

Возвращает: [`Option[PageUsersOnlineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_online_response.nim)

## Example

[inline-code-attrs-start title = 'Пример getOnlineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetOnlineUsersOptions()
let (onlineUsersOpt, httpResp) = client.getOnlineUsers(tenantId = "my-tenant-123", urlId = "news/article-title", options = opts)
if onlineUsersOpt.isSome:
  let onlineUsers = onlineUsersOpt.get()
  echo onlineUsers
[inline-code-end]