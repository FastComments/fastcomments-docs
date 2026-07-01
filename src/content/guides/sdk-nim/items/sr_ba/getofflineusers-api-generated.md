Prethodni komentatori na stranici koji NISU trenutno online. Sortirano po displayName.  
Koristite ovo nakon što iscrpite /users/online za prikaz sekcije „Members”.  
Kursor paginacija po commenterName: server prolazi kroz djelimični {tenantId, urlId, commenterName} indeks od afterName naprijed putem $gt, bez $skip troška.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | GetOfflineUsersOptions | No |  |

## Response

Returns: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Example

[inline-code-attrs-start title = 'getOfflineUsers Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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