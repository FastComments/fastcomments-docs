## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|----------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## Antwort

Rückgabe: [`Option[GetModeratorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_moderator_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getModerator Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (moderatorOpt, httpResponse) = client.getModerator(tenantId = "my-tenant-123", id = "moderator-456")
if moderatorOpt.isSome:
  let moderator = moderatorOpt.get()
  discard moderator
[inline-code-end]