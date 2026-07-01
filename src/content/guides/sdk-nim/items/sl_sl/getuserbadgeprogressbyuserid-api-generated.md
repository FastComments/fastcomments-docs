## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |

## Odgovor

Vrne: [`Option[APIGetUserBadgeProgressResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_user_badge_progress_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getUserBadgeProgressByUserId'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (badgeProgressOpt, httpResp) = client.getUserBadgeProgressByUserId(tenantId = "my-tenant-123", userId = "user-456")
if badgeProgressOpt.isSome:
  let progress = badgeProgressOpt.get()
  echo progress
[inline-code-end]