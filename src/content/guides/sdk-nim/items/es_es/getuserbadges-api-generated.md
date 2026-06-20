## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| userId | string | No |  |
| badgeId | string | No |  |
| displayedOnComments | bool | No |  |
| limit | float64 | No |  |
| skip | float64 | No |  |

## Respuesta

Devuelve: [`Option[APIGetUserBadgesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_user_badges_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getUserBadges'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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