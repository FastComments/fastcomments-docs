## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| id | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[UpdateUserBadge_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_badge200response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład deleteUserBadge'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteUserBadge(tenantId = "my-tenant-123", id = "badge-456")
if response.isSome:
  let updated = response.get()
  discard updated
[inline-code-end]

---