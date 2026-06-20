Attualmente i visualizzatori online di una pagina: persone la cui sessione websocket è sottoscritta alla pagina in questo momento.
Restituisce anonCount + totalCount (iscritti alla room in generale, inclusi i visualizzatori anonimi che non elenchiamo).

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| urlId | string | Sì |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Risposta

Restituisce: [`Option[PageUsersOnlineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_online_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di getOnlineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getOnlineUsers(tenantId = "my-tenant-123", urlId = "news/politics/top-story", afterName = "", afterUserId = "")
if response.isSome:
  let page = response.get()
  echo "Received online users page:"
  echo page
else:
  echo "No online users returned. HTTP status: ", httpResponse.statusCode
[inline-code-end]

---