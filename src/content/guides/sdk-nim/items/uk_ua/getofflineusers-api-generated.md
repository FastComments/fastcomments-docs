---
Попередні коментатори на сторінці, які **НЕ** перебувають онлайн. Відсортовано за `displayName`.  
Використовуйте це після використання `/users/online`, щоб відобразити розділ «Members».  
Пагінація курсором за `commenterName`: сервер проходить частковий `{tenantId, urlId, commenterName}` індекс після `afterName` вперед за допомогою `$gt`, без витрат `$skip`.

## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | GetOfflineUsersOptions | No |  |

## Відповідь

Returns: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getOfflineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---