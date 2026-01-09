## Parametri

| Name | Type | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| userId | string | Ne |  |
| badgeId | string | Ne |  |
| displayedOnComments | bool | Ne |  |
| limit | float64 | Ne |  |
| skip | float64 | Ne |  |

## Odgovor

Vraća: [`Option[GetUserBadges_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_badges200response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer getUserBadges'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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