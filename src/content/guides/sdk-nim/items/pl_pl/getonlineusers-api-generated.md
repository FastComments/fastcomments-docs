Obecnie online widzowie strony: osoby, których sesja websocket jest obecnie zasubskrybowana do tej strony.
Zwraca anonCount + totalCount (subskrybenci w całym pokoju, w tym anonimowi widzowie, których nie wymieniamy).

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| urlId | string | Tak |  |
| afterName | string | Nie |  |
| afterUserId | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[PageUsersOnlineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_online_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getOnlineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getOnlineUsers(tenantId = "my-tenant-123", urlId = "news/politics/top-story", afterName = "", afterUserId = "")
if response.isSome:
  let page = response.get()
  echo "Received online users page:"
  echo page
else:
  echo "No online users returned. HTTP status: ", httpResponse.statusCode
[inline-code-end]

---