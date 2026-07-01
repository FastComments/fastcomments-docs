Prejšnji komentatorji na strani, ki NI trenutno v povezavi. Razvrščeni po displayName.  
Uporabite to po izčrpavanju /users/online za prikaz odseka "Members".  
Cursor paginacija po commenterName: strežnik prehaja delni {tenantId, urlId, commenterName} indeks od afterName naprej prek $gt, brez stroška $skip.

## Parameters

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | GetOfflineUsersOptions | No |  |

## Response

Vrne: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getOfflineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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