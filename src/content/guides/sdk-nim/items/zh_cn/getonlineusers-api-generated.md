Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
返回 anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).  

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | GetOnlineUsersOptions | No |  |

## 响应

返回: [`Option[PageUsersOnlineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_online_response.nim)

## 示例

[inline-code-attrs-start title = 'getOnlineUsers 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
let opts = GetOnlineUsersOptions()  
let (onlineUsersOpt, httpResp) = client.getOnlineUsers(tenantId = "my-tenant-123", urlId = "news/article-title", options = opts)  
if onlineUsersOpt.isSome:  
  let onlineUsers = onlineUsersOpt.get()  
  echo onlineUsers  
[inline-code-end]