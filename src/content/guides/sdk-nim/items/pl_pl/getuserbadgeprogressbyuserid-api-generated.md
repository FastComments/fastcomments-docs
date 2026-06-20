## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |

## Odpowiedź

Zwraca: [`Option[APIGetUserBadgeProgressResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_user_badge_progress_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getUserBadgeProgressByUserId'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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