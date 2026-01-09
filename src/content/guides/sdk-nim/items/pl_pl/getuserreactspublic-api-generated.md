## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| postIds | seq[string] | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[GetUserReactsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_reacts_public200response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getUserReactsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserReactsPublic(tenantId = "my-tenant-123", postIds = @[], sso = "")
if response.isSome:
  let reacts = response.get()
  discard reacts
[inline-code-end]

---