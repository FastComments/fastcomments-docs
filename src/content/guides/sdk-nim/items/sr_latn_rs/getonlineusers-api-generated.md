Trenutno onlajn posmatrači stranice: ljudi čija je websocket sesija trenutno pretplaćena na stranicu. Vraća anonCount + totalCount (pretplatnici po sobi, uključujući anonimne posmatrače koje ne nabrajamo).

## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| options | GetOnlineUsersOptions | Ne |  |

## Odgovor

Vraća: [`Option[PageUsersOnlineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_online_response.nim)

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