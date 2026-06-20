Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Response

Returns: [`Option[PageUsersOnlineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_online_response.nim)

## Example

[inline-code-attrs-start title = 'getOnlineUsers Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getOnlineUsers(tenantId = "my-tenant-123", urlId = "news/politics/top-story", afterName = "", afterUserId = "")
if response.isSome:
  let page = response.get()
  echo "Received online users page:"
  echo page
else:
  echo "No online users returned. HTTP status: ", httpResponse.statusCode
[inline-code-end]
