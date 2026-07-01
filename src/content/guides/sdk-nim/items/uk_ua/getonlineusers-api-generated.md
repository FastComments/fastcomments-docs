---
Поточні онлайн переглядачі сторінки: люди, чия сесія вебсокета підписана на сторінку в даний момент.  
Повертає anonCount + totalCount (підписники всього простору, включаючи анонімних глядачів, яких ми не перераховуємо).

## Parameters

| Назва | Тип | Обов’язково | Опис |
|------|------|--------------|------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | GetOnlineUsersOptions | No |  |

## Response

Повертає: [`Option[PageUsersOnlineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_online_response.nim)

## Example

[inline-code-attrs-start title = 'Приклад getOnlineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetOnlineUsersOptions()
let (onlineUsersOpt, httpResp) = client.getOnlineUsers(tenantId = "my-tenant-123", urlId = "news/article-title", options = opts)
if onlineUsersOpt.isSome:
  let onlineUsers = onlineUsersOpt.get()
  echo onlineUsers
[inline-code-end]

---