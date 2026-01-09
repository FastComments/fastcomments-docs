## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| userId | string | No |  |
| badgeId | string | No |  |
| displayedOnComments | bool | No |  |
| limit | float64 | No |  |
| skip | float64 | No |  |

## Risposta

Restituisce: [`Option[GetUserBadges_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_badges200response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di getUserBadges'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserBadges(
  tenantId = "my-tenant-123",
  userId = "user-789",
  badgeId = "top-commenter",
  displayedOnComments = true,
  limit = 50.0,
  skip = 0.0
)

if response.isSome:
  let badges = response.get()
  echo "Retrieved badges: ", $badges
[inline-code-end]