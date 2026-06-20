## Parametri

| Name | Type | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| userId | string | Ne |  |
| badgeId | string | Ne |  |
| displayedOnComments | bool | Ne |  |
| limit | float64 | Ne |  |
| skip | float64 | Ne |  |

## Odgovor

Vrne: [`Option[APIGetUserBadgesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_user_badges_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getUserBadges'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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