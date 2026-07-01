## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|---------------|--------------|
| tenantId | string | Ja |  |
| createUserBadgeParams | CreateUserBadgeParams | Nein |  |

## Antwort

Rückgabe: [`Option[APICreateUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_create_user_badge_response.nim)

## Beispiel

[inline-code-attrs-start title = 'createUserBadge Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (badgeRespOpt, httpResp) = client.createUserBadge(tenantId = "my-tenant-123", createUserBadgeParams = default(CreateUserBadgeParams))
if badgeRespOpt.isSome:
  let badgeResp = badgeRespOpt.get()
[inline-code-end]