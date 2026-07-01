## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetUserReactsPublicOptions | No |  |

## Réponse

Renvoie : [`Option[UserReactsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_user_reacts_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getUserReactsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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