## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Нет |  |

## Ответ

Возвращает: [`Option[GetUserBadgeProgressById_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_badge_progress_by_id200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getUserBadgeProgressById'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserBadgeProgressById(tenantId = "my-tenant-123", id = "editor-badge-42")
if response.isSome:
  let badgeProgress = response.get()
  echo "Badge progress received:"
  echo badgeProgress
else:
  echo "No badge progress found for tenant 'my-tenant-123' and id 'editor-badge-42'"
  echo httpResponse
[inline-code-end]

---