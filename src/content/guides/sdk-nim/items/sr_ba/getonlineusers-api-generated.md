---
Trenutni gledatelji stranice: osobe čija je websocket sesija pretplaćena na stranicu u ovom trenutku.
Vraća anonCount + totalCount (pretplatnici u cijeloj sobi, uključujući anonimne gledatelje koje ne izlistavamo).

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | GetOnlineUsersOptions | No |  |

## Odgovor

Vraća: [`Option[PageUsersOnlineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_online_response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer getOnlineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetOnlineUsersOptions()
let (onlineUsersOpt, httpResp) = client.getOnlineUsers(tenantId = "my-tenant-123", urlId = "news/article-title", options = opts)
if onlineUsersOpt.isSome:
  let onlineUsers = onlineUsersOpt.get()
  echo onlineUsers
[inline-code-end]

---