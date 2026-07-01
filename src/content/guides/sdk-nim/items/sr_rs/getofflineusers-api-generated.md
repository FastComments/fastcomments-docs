Prethodni komentatori na stranici koji NISU trenutno online. Sortirano po displayName.  
Koristite ovo nakon što iscrpite /users/online da prikažete sekciju „Members“.  
Cursor paginacija po commenterName: server prolazi kroz parcijalni {tenantId, urlId, commenterName} indeks od afterName napred putem $gt, bez troška $skip.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | GetOfflineUsersOptions | No |  |

## Response

Vraća: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

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