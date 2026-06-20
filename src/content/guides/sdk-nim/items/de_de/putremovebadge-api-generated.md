## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| badgeId | string | Nein |  |
| userId | string | Nein |  |
| commentId | string | Ja |  |
| broadcastId | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`Option[RemoveUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_remove_user_badge_response.nim)

## Beispiel

[inline-code-attrs-start title = 'putRemoveBadge Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.putRemoveBadge(badgeId = "verified-journalist",
  userId = "user-7890",
  commentId = "comment-98765",
  broadcastId = "",
  sso = "")

if response.isSome:
  let removeResp = response.get()
  discard removeResp
else:
  discard httpResponse
[inline-code-end]

---