---
Zbiorcze informacje o użytkownikach dla najemcy. Dla podanych userIds zwraca informacje wyświetlane z User / SSOUser.
Wykorzystywane przez widżet komentarzy do wzbogacenia użytkowników, którzy właśnie pojawili się w wyniku zdarzenia obecności.
Brak kontekstu strony: prywatność jest egzekwowana jednolicie (prywatne profile są maskowane).

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | No |  |

## Odpowiedź

Zwraca: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getUsersInfo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user-42,user-87")
if response.isSome:
  let usersInfo = response.get()
  echo "Retrieved users info:", usersInfo
else:
  echo "No users info returned. HTTP status:", httpResponse.status
[inline-code-end]

---