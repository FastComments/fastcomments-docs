## Параметри

| Назва | Тип | Обов'язкове | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| userId | string | Ні |  |
| badgeId | string | Ні |  |
| displayedOnComments | bool | Ні |  |
| limit | float64 | Ні |  |
| skip | float64 | Ні |  |

## Відповідь

Повертає: [`Option[GetUserBadges_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_badges200response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getUserBadges'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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