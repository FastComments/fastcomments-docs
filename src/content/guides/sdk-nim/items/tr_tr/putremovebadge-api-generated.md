## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| badgeId | string | Hayır |  |
| userId | string | Hayır |  |
| commentId | string | Evet |  |
| broadcastId | string | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Dönüş: [`Option[RemoveUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_remove_user_badge_response.nim)

## Örnek

[inline-code-attrs-start title = 'putRemoveBadge Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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