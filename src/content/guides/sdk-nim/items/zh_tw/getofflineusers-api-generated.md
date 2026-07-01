過去在此頁面留言的使用者，且目前未在線上。依 displayName 排序。  
在耗盡 /users/online 後使用，以呈現「Members」區段。  
在 commenterName 上的游標分頁：伺服器從 afterName 向前遍歷部分 {tenantId, urlId, commenterName} 索引，使用 $gt 前進，無 $skip 成本。

## Parameters

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | GetOfflineUsersOptions | No |  |

## Response

Returns: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Example

[inline-code-attrs-start title = 'getOfflineUsers 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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