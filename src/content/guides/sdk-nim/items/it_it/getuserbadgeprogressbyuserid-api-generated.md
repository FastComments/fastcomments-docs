## Parametri

| Nome | Tipo | Richiesto | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| userId | string | No |  |

## Risposta

Restituisce: [`Option[GetUserBadgeProgressById_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_badge_progress_by_id200response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio getUserBadgeProgressByUserId'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserBadgeProgressByUserId(tenantId = "my-tenant-123", userId = "user-456")
if response.isSome:
  let badgeProgress = response.get()
  echo "Badge progress retrieved for user-456"
else:
  echo "No badge progress found, HTTP status: ", $httpResponse.status
[inline-code-end]

---