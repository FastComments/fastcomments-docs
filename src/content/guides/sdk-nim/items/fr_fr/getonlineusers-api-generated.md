Actuellement, les spectateurs en ligne d'une page : personnes dont la session websocket est abonnée à la page en ce moment.
Retourne anonCount + totalCount (abonnés au salon, y compris les spectateurs anonymes que nous n'énumérons pas).

## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Réponse

Retourne : [`Option[PageUsersOnlineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_online_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple d\'utilisation de getOnlineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getOnlineUsers(tenantId = "my-tenant-123", urlId = "news/politics/top-story", afterName = "", afterUserId = "")
if response.isSome:
  let page = response.get()
  echo "Received online users page:"
  echo page
else:
  echo "No online users returned. HTTP status: ", httpResponse.statusCode
[inline-code-end]