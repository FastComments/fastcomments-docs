## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetUserBadgeProgressListOptions | No |  |

## Відповідь

Повертає: [`Option[APIGetUserBadgeProgressListResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_user_badge_progress_list_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getUserBadgeProgressList'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserBadgeProgressList(tenantId = "my-tenant-123", options = GetUserBadgeProgressListOptions())
if response.isSome:
  let badgeProgress = response.get()
[inline-code-end]

---