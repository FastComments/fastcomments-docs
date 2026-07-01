## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| badgeId | string | Ні |  |
| options | PutAwardBadgeOptions | Ні |  |

## Відповідь

Повертає: [`Option[AwardUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_award_user_badge_response.nim)

## Приклад

[inline-code-attrs-start title = 'putAwardBadge Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.putAwardBadge(
  tenantId = "my-tenant-123",
  badgeId = "gold-badge",
  options = PutAwardBadgeOptions()
)

if response.isSome:
  let award = response.get()
[inline-code-end]

---