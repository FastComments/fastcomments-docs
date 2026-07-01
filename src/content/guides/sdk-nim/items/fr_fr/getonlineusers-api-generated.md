---
Visiteurs actuellement en ligne d'une page : personnes dont la session websocket est abonnée à la page en ce moment.  
Renvoie anonCount + totalCount (abonnés à l’ensemble de la salle, y compris les visiteurs anonymes que nous n’énumérons pas).

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| options | GetOnlineUsersOptions | Non |  |

## Réponse

Renvoie : [`Option[PageUsersOnlineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_online_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getOnlineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetOnlineUsersOptions()
let (onlineUsersOpt, httpResp) = client.getOnlineUsers(tenantId = "my-tenant-123", urlId = "news/article-title", options = opts)
if onlineUsersOpt.isSome:
  let onlineUsers = onlineUsersOpt.get()
  echo onlineUsers
[inline-code-end]

---