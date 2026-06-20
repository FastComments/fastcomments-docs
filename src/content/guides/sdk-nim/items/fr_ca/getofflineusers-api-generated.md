Commentateurs précédents sur la page qui NE sont PAS actuellement en ligne. Triés par displayName.
Utilisez ceci après avoir épuisé /users/online pour afficher une section «Membres».
Pagination par curseur sur commenterName : le serveur parcourt l'index partiel {tenantId, urlId, commenterName}
index depuis afterName vers l'avant via $gt, sans coût lié à $skip.

## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| afterName | string | Non |  |
| afterUserId | string | Non |  |

## Réponse

Renvoie : [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getOfflineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getOfflineUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-how-to-code",
  afterName = "",
  afterUserId = ""
)

if response.isSome:
  let offlinePage = response.get()
  echo "Received offline users page"
  discard httpResponse.statusCode
[inline-code-end]

---