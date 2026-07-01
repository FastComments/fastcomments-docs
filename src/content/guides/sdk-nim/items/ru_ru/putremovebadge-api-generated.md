## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| badgeId | string | No |  |
| options | PutRemoveBadgeOptions | No |  |

## Ответ

Возвращает: [`Option[RemoveUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_remove_user_badge_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример putRemoveBadge'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.putRemoveBadge(
  tenantId = "my-tenant-123",
  badgeId = "badge-456",
  options = PutRemoveBadgeOptions()
)

if maybeResp.isSome:
  let badgeResp = maybeResp.get()
[inline-code-end]