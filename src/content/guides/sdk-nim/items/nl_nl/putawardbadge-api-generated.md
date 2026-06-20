## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| badgeId | string | Nee |  |
| userId | string | Nee |  |
| commentId | string | Ja |  |
| broadcastId | string | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`Option[AwardUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_award_user_badge_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'putAwardBadge Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.putAwardBadge(
  badgeId = "gold-contributor",
  userId = "user-8723",
  commentId = "cmt-54a3b2",
  broadcastId = "",
  sso = ""
)
if response.isSome:
  let award = response.get()
  echo "Awarded badge received"
else:
  echo "No award response"
[inline-code-end]

---