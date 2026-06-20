## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| badgeId | string | Nie |  |
| userId | string | Nie |  |
| commentId | string | Tak |  |
| broadcastId | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[AwardUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_award_user_badge_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia putAwardBadge'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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