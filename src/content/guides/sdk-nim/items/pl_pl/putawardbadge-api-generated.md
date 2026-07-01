## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| badgeId | string | No |  |
| options | PutAwardBadgeOptions | No |  |

## Odpowiedź

Zwraca: [`Option[AwardUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_award_user_badge_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład putAwardBadge'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.putAwardBadge(
  tenantId = "my-tenant-123",
  badgeId = "gold-badge",
  options = PutAwardBadgeOptions()
)

if response.isSome:
  let award = response.get()
[inline-code-end]