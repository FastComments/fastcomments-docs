Past commenters on the page who are NOT currently online. Sorted by displayName.  
Користите ово након што исцрпите /users/online да прикажете одељак „Чланови“.

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
Курсорска пагинација на commenterName: сервер пролази парцијални {tenantId, urlId, commenterName} индекс од afterName унапред помоћу $gt, без трошка $skip.

## Parameters

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | GetOfflineUsersOptions | No |  |

## Response

Returns: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Example

[inline-code-attrs-start title = 'Primer getOfflineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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