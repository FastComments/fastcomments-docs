## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createUserBadgeParams | CreateUserBadgeParams | Не |  |

## Одговор

Враћа: [`Option[APICreateUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_create_user_badge_response.nim)

## Пример

[inline-code-attrs-start title = 'createUserBadge Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createUserBadge(
  tenantId = "my-tenant-123",
  createUserBadgeParams = CreateUserBadgeParams(
    userId = "user-456",
    badgeId = "top-commenter",
    reason = "Top commenter for June 2026",
    awardedBy = "mod-team",
    metadata = @["news","engagement"]
  )
)
if response.isSome:
  let badgeResp = response.get()
  discard badgeResp
[inline-code-end]

---