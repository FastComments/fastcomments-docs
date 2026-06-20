## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| sso | string | 否 |  |

## 回應

回傳: [`Option[GetBannedUsersFromCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_banned_users_from_comment_response.nim)

## 範例

[inline-code-attrs-start title = 'getBanUsersFromComment 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getBanUsersFromComment(commentId = "comment-98765", sso = "")
if response.isSome:
  let bannedResp = response.get()
  discard bannedResp
else:
  echo "No banned users found or request failed"
[inline-code-end]

---