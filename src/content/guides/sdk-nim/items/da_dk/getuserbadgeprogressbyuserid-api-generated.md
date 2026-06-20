## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| userId | string | Nej |  |

## Svar

Returnerer: [`Option[APIGetUserBadgeProgressResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_user_badge_progress_response.nim)

## Eksempel

[inline-code-attrs-start title = 'getUserBadgeProgressByUserId Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let userId = "user-456"
let (response, httpResponse) = client.getUserBadgeProgressByUserId(tenantId = tenantId, userId = userId)
if response.isSome:
  let badgeProgress = response.get()
  echo "Badge progress retrieved for ", userId
  discard badgeProgress
[inline-code-end]

---