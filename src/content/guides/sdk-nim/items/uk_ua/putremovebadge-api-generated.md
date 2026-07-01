## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| badgeId | string | Ні |  |
| options | PutRemoveBadgeOptions | Ні |  |

## Відповідь

Повертає: [`Option[RemoveUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_remove_user_badge_response.nim)

## Приклад

[inline-code-attrs-start title = 'putRemoveBadge Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.putRemoveBadge(
  tenantId = "my-tenant-123",
  badgeId = "badge-456",
  options = PutRemoveBadgeOptions()
)

if maybeResp.isSome:
  let badgeResp = maybeResp.get()
[inline-code-end]

---