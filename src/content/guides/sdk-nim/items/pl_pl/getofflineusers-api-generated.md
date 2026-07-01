---
Poprzedni komentatorzy na stronie, którzy NIE są aktualnie online. Posortowani według displayName.  
Użyj tego po wyczerpaniu /users/online, aby wyrenderować sekcję "Members".  
Paginacja kursora na commenterName: serwer przegląda częściowy {tenantId, urlId, commenterName} indeks od afterName naprzód przy użyciu $gt, bez kosztu $skip.

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | GetOfflineUsersOptions | No |  |

## Odpowiedź

Zwraca: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getOfflineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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