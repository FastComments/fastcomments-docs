Commentateurs passés sur la page qui ne sont PAS actuellement en ligne. Triés par displayName.
À utiliser après avoir épuisé /users/online pour afficher une section « Membres ».
Pagination par curseur sur commenterName : le serveur parcourt l'index partiel {tenantId, urlId, commenterName}
à partir de afterName via $gt, sans coût de $skip.

## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| options | GetOfflineUsersOptions | Non |  |

## Réponse

Retourne : [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getOfflineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (offlineResp, httpResponse) = client.getOfflineUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  options = GetOfflineUsersOptions()
)
if offlineResp.isSome:
  let offline = offlineResp.get()
  echo offline)
[inline-code-end]