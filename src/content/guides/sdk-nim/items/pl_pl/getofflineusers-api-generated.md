Poprzedni komentujący na stronie, którzy NIE są obecnie online. Posortowane według displayName.
Użyj tego po wyczerpaniu /users/online, aby wyświetlić sekcję "Członkowie".
Paginacja kursorowa po commenterName: serwer przeszukuje częściowy {tenantId, urlId, commenterName}
indeks od afterName w przód przy użyciu $gt, bez kosztu $skip.

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| urlId | string | Tak |  |
| afterName | string | Nie |  |
| afterUserId | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getOfflineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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