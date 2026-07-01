## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| userId | string | Nein |  |

## Antwort

Gibt zurück: [`Option[APIGetUserBadgeProgressResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_user_badge_progress_response.nim)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getUserBadgeProgressByUserId'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (badgeProgressOpt, httpResp) = client.getUserBadgeProgressByUserId(tenantId = "my-tenant-123", userId = "user-456")
if badgeProgressOpt.isSome:
  let progress = badgeProgressOpt.get()
  echo progress
[inline-code-end]