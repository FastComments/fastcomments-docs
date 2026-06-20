## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| badgeId | string | Ne |  |
| userId | string | Ne |  |
| commentId | string | Da |  |
| broadcastId | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`Option[RemoveUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_remove_user_badge_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer putRemoveBadge'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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