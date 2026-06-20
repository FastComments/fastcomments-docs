---
Предишни коментатори на страницата, които НЕ са в момента онлайн. Подредени по displayName.
Използвайте това след като сте изчерпали /users/online, за да визуализирате секция "Членове".
Курсорна пагинация по commenterName: сървърът обхожда частичния {tenantId, urlId, commenterName}
индекс от afterName напред чрез $gt, без разход за $skip.

## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| afterName | string | Не |  |
| afterUserId | string | Не |  |

## Отговор

Връща: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за getOfflineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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