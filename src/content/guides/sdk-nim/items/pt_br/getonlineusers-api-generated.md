Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | GetOnlineUsersOptions | No |  |

## Response

Returns: [`Option[PageUsersOnlineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_online_response.nim)

## Example

[inline-code-attrs-start title = 'Exemplo getOnlineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
let opts = GetOnlineUsersOptions()  
let (onlineUsersOpt, httpResp) = client.getOnlineUsers(tenantId = "my-tenant-123", urlId = "news/article-title", options = opts)  
if onlineUsersOpt.isSome:  
  let onlineUsers = onlineUsersOpt.get()  
  echo onlineUsers  
[inline-code-end]