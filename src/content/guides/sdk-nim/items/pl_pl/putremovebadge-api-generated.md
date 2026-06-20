## Parametry

| Name | Type | Wymagane | Opis |
|------|------|----------|-------------|
| badgeId | string | Nie |  |
| userId | string | Nie |  |
| commentId | string | Tak |  |
| broadcastId | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[RemoveUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_remove_user_badge_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład putRemoveBadge'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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