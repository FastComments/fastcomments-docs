## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| badgeId | string | Ne |  |
| options | PutAwardBadgeOptions | Ne |  |

## Odgovor

Vraća: [`Option[AwardUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_award_user_badge_response.nim)

## Primer

[inline-code-attrs-start title = 'putAwardBadge Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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