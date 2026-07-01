Prethodni komentatori na stranici koji NIJE trenutno online. Sortirano po displayName.  
Koristite ovo nakon što ste iscrpili /users/online za prikaz sekcije „Članovi“.  
Kursor paginacija po commenterName: poslužitelj prolazi kroz djelomični {tenantId, urlId, commenterName}  
indeks od afterName naprijed putem $gt, bez troška $skip.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | GetOfflineUsersOptions | No |  |

## Odgovor

Vraća: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer getOfflineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---