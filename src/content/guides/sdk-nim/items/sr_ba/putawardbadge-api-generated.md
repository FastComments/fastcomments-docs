## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| badgeId | string | Ne |  |
| userId | string | Ne |  |
| commentId | string | Da |  |
| broadcastId | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`Option[AwardUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_award_user_badge_response.nim)

## Primjer

[inline-code-attrs-start title = 'putAwardBadge Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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