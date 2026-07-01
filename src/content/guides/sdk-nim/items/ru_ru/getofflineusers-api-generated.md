Past commenters on the page who are NOT currently online. Sorted by displayName.  
**Russian:** Прошлые комментаторы на странице, которые НЕ находятся в сети в данный момент. Отсортированы по displayName.  

Use this after exhausting /users/online to render a "Members" section.  
**Russian:** Используйте это после исчерпания /users/online для отображения раздела "Members".  

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
**Russian:** Курсорная пагинация по commenterName: сервер проходит частичный {tenantId, urlId, commenterName} индекс от afterName вперёд через $gt, без затрат на $skip.  

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | GetOfflineUsersOptions | No |  |

## Response

Возвращает: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Example

[inline-code-attrs-start title = 'Пример getOfflineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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