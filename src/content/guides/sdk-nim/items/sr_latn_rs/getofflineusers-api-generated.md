Past komentatori na stranici koji NISU trenutno onlajn. Sortirano po displayName.  
Koristite ovo nakon što iskoristite /users/online da prikažete sekciju „Members”.  
Kursor paginacija po commenterName: server prolazi delimični {tenantId, urlId, commenterName} indeks od afterName napred putem $gt, bez troška $skip.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| options | GetOfflineUsersOptions | Ne |  |

## Odgovor

Returns: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Primer

[inline-code-attrs-start title = 'getOfflineUsers Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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