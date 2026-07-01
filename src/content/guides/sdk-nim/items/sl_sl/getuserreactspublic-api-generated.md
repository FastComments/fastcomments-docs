## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetUserReactsPublicOptions | No |  |

## Odgovor

Vrne: [`Option[UserReactsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_user_reacts_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getUserReactsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetUserReactsPublicOptions(
  limit = 20,
  offset = 0,
  includeDeleted = false
)

let (response, httpResponse) = client.getUserReactsPublic(
  tenantId = "my-tenant-123",
  options = opts
)

if response.isSome:
  let userReacts = response.get()
[inline-code-end]