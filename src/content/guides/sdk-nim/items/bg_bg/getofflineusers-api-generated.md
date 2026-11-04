Минали коментатори на страницата, които в момента НЕ са онлайн. Подредени по displayName.
Използвайте това след изчерпване на /users/online, за да визуализирате секция "Members".
Пагинация с курсор по commenterName: сървърът обхожда частичния {tenantId, urlId, commenterName}
индекс от afterName напред чрез $gt, без разход за $skip.

## Параметри

| Име | Тип | Задължителен | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| options | GetOfflineUsersOptions | Не |  |

## Отговор

Връща: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за getOfflineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (offlineResp, httpResponse) = client.getOfflineUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  options = GetOfflineUsersOptions()
)
if offlineResp.isSome:
  let offline = offlineResp.get()
  echo offline)
[inline-code-end]