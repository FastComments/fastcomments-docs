Past commenters on the page who are NOT currently online. Sorted by displayName.  
Commentateurs précédents sur la page qui ne sont PAS actuellement en ligne. Triés par displayName.

Use this after exhausting /users/online to render a "Members" section.  
Utilisez ceci après avoir épuisé /users/online pour afficher une section « Membres ».

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
Pagination du curseur sur commenterName : le serveur parcourt le fragment {tenantId, urlId, commenterName} à partir de afterName vers l’avant via $gt, sans coût $skip.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | GetOfflineUsersOptions | No |  |

## Response

Returns: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Example

[inline-code-attrs-start title = 'Exemple getOfflineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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