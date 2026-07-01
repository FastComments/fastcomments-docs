---
Visualizzatori attualmente online di una pagina: persone la cui sessione websocket è iscritta alla pagina in questo momento.  
Restituisce anonCount + totalCount (sottoscrittori dell’intera stanza, inclusi visualizzatori anonimi che non elenchiamo).

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| urlId | string | Sì |  |
| options | GetOnlineUsersOptions | No |  |

## Risposta

Restituisce: [`Option[PageUsersOnlineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_online_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio getOnlineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetOnlineUsersOptions()
let (onlineUsersOpt, httpResp) = client.getOnlineUsers(tenantId = "my-tenant-123", urlId = "news/article-title", options = opts)
if onlineUsersOpt.isSome:
  let onlineUsers = onlineUsersOpt.get()
  echo onlineUsers
[inline-code-end]