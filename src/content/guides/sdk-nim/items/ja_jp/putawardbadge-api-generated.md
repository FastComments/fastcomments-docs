---
## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| badgeId | string | No |  |
| userId | string | No |  |
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## レスポンス

返却: [`Option[AwardUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_award_user_badge_response.nim)

## 例

[inline-code-attrs-start title = 'putAwardBadge の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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