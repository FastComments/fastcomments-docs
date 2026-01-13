## Parametry

| Name | Type | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| urlId | string | Tak |  |
| userId | string | Nie |  |
| anonUserId | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[GetVotesForUser_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_votes_for_user200response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getVotesForUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getVotesForUser(
  tenantId = "my-tenant-123",
  urlId = "news/economy-update-2026",
  userId = "user-789",
  anonUserId = ""
)

if response.isSome:
  let votes = response.get()
  discard votes
[inline-code-end]

---