Prejšnji komentatorji na strani, ki trenutno niso online. Razvrščeno po displayName.
Uporabite to po izčrpanju /users/online, da prikažete razdelek "Člani".
Straničenje z kurzorjem po commenterName: strežnik prehaja parcialni indeks {tenantId, urlId, commenterName}
od afterName naprej z uporabo $gt, brez stroška $skip.

## Parameters

| Ime | Tip | Zahtevano | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| afterName | string | Ne |  |
| afterUserId | string | Ne |  |

## Odziv

Vrne: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getOfflineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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