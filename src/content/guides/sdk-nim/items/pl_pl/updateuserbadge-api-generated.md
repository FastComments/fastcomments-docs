## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| id | string | Nie |  |
| updateUserBadgeParams | UpdateUserBadgeParams | Nie |  |

## Odpowiedź

Zwraca: [`Option[APIEmptySuccessResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_success_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład updateUserBadge'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = UpdateUserBadgeParams()
let (maybeResp, httpResp) = client.updateUserBadge(tenantId = "my-tenant-123", id = "user-456", updateUserBadgeParams = params)
if maybeResp.isSome:
  let success = maybeResp.get()
[inline-code-end]