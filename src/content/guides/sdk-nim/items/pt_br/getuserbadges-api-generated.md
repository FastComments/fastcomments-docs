## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| userId | string | Não |  |
| badgeId | string | Não |  |
| displayedOnComments | bool | Não |  |
| limit | float64 | Não |  |
| skip | float64 | Não |  |

## Resposta

Retorna: [`Option[APIGetUserBadgesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_user_badges_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getUserBadges'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserBadges(
  tenantId = "my-tenant-123",
  userId = "user-9876",
  badgeId = "top-commenter",
  displayedOnComments = true,
  limit = 20.0,
  skip = 0.0
)

if response.isSome:
  let badges = response.get()
  echo "Badges response:", badges
else:
  echo "No badges found (HTTP status: ", httpResponse.status, ")"
[inline-code-end]

---