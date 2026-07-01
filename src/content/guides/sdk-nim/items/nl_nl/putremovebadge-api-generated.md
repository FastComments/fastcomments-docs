## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Yes |  |
| badgeId | string | No |  |
| options | PutRemoveBadgeOptions | No |  |

## Respons

Retourneert: [`Option[RemoveUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_remove_user_badge_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'putRemoveBadge Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.putRemoveBadge(
  tenantId = "my-tenant-123",
  badgeId = "badge-456",
  options = PutRemoveBadgeOptions()
)

if maybeResp.isSome:
  let badgeResp = maybeResp.get()
[inline-code-end]