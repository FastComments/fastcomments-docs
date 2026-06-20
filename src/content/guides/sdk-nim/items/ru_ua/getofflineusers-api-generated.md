---
Пользователи, которые ранее комментировали на странице, но в данный момент НЕ в сети. Отсортировано по displayName.
Используйте это после обращения к /users/online, чтобы отобразить раздел «Участники».
Постраничная навигация курсором по commenterName: сервер проходит по частичному индексу {tenantId, urlId, commenterName} от afterName вперёд с помощью $gt, без затрат $skip.

## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| afterName | string | Нет |  |
| afterUserId | string | Нет |  |

## Ответ

Возвращает: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getOfflineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getOfflineUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-how-to-code",
  afterName = "",
  afterUserId = ""
)

if response.isSome:
  let offlinePage = response.get()
  echo "Received offline users page"
  discard httpResponse.statusCode
[inline-code-end]

---