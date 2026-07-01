Trenutno povezana gledalci strani: ljudje, katerih websocket seja je trenutno naročena na stran.  
Vrne anonCount + totalCount (pretplatnike po celotnem prostoru, vključno z anonimnimi gledalci, ki jih ne naštejemo).

## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| options | GetOnlineUsersOptions | Ne |  |

## Odgovor

Vrne: [`Option[PageUsersOnlineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_online_response.nim)

## Primer

[inline-code-attrs-start title = 'getOnlineUsers Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetOnlineUsersOptions()
let (onlineUsersOpt, httpResp) = client.getOnlineUsers(tenantId = "my-tenant-123", urlId = "news/article-title", options = opts)
if onlineUsersOpt.isSome:
  let onlineUsers = onlineUsersOpt.get()
  echo onlineUsers
[inline-code-end]

---