目前在線的頁面觀眾：指其 WebSocket 會話目前已訂閱該頁面的使用者。  
返回 anonCount + totalCount（全站訂閱者數，包括我們未列舉的匿名觀眾）。

## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| options | GetOnlineUsersOptions | 否 |  |

## 回應

返回：[`Option[PageUsersOnlineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_online_response.nim)

## 範例

[inline-code-attrs-start title = 'getOnlineUsers 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetOnlineUsersOptions()
let (onlineUsersOpt, httpResp) = client.getOnlineUsers(tenantId = "my-tenant-123", urlId = "news/article-title", options = opts)
if onlineUsersOpt.isSome:
  let onlineUsers = onlineUsersOpt.get()
  echo onlineUsers
[inline-code-end]