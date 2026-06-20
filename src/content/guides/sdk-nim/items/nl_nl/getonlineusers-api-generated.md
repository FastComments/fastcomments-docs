Momenteel online kijkers van een pagina: mensen wiens websocket session op dit moment op de pagina geabonneerd is.
Retourneert anonCount + totalCount (kamer-brede abonnees, inclusief anonieme kijkers die we niet opsommen).

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| afterName | string | Nee |  |
| afterUserId | string | Nee |  |

## Response

Retourneert: [`Option[PageUsersOnlineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_online_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getOnlineUsers Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getOnlineUsers(tenantId = "my-tenant-123", urlId = "news/politics/top-story", afterName = "", afterUserId = "")
if response.isSome:
  let page = response.get()
  echo "Received online users page:"
  echo page
else:
  echo "No online users returned. HTTP status: ", httpResponse.statusCode
[inline-code-end]