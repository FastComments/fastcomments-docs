## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| badgeId | string | Nee |  |
| userId | string | Nee |  |
| commentId | string | Ja |  |
| broadcastId | string | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`Option[RemoveUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_remove_user_badge_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'putRemoveBadge Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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