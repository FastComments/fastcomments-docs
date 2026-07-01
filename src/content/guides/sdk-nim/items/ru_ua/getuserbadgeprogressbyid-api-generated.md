## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## Ответ

Возвращает: [`Option[APIGetUserBadgeProgressResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_user_badge_progress_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getUserBadgeProgressById'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (badgeProgressOpt, httpResp) = client.getUserBadgeProgressById(tenantId = "my-tenant-123", id = "badge-456")
if badgeProgressOpt.isSome:
  let badgeProgress = badgeProgressOpt.get()
  echo badgeProgress
[inline-code-end]