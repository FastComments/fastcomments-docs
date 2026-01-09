## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| userId | string | Не |  |
| limit | float64 | Не |  |
| skip | float64 | Не |  |

## Одговор

Враћа: [`Option[GetUserBadgeProgressList_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_badge_progress_list200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getUserBadgeProgressList'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserBadgeProgressList(tenantId = "my-tenant-123", userId = "user-9823", limit = 25.0, skip = 0.0)
if response.isSome:
  let badgeProgress = response.get()
  echo "Badge progress received:", badgeProgress
else:
  echo "No badge progress. HTTP response:", httpResponse.status
[inline-code-end]

---