## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| badgeId | string | 否 |  |
| userId | string | 否 |  |
| commentId | string | 是 |  |
| broadcastId | string | 否 |  |
| sso | string | 否 |  |

## 回傳

回傳: [`Option[RemoveUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_remove_user_badge_response.nim)

## 範例

[inline-code-attrs-start title = 'putRemoveBadge 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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