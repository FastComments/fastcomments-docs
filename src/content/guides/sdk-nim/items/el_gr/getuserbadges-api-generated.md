## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| userId | string | Όχι |  |
| badgeId | string | Όχι |  |
| displayedOnComments | bool | Όχι |  |
| limit | float64 | Όχι |  |
| skip | float64 | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[GetUserBadges_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_badges200response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUserBadges'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---