## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| banEmail | bool | 否 |  |
| banEmailDomain | bool | 否 |  |
| banIP | bool | 否 |  |
| deleteAllUsersComments | bool | 否 |  |
| bannedUntil | string | 否 |  |
| isShadowBan | bool | 否 |  |
| updateId | string | 否 |  |
| banReason | string | 否 |  |
| sso | string | 否 |  |

## 回傳

回傳: [`Option[BanUserFromCommentResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_ban_user_from_comment_result.nim)

## 範例

[inline-code-attrs-start title = 'postBanUserFromComment 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postBanUserFromComment(
  commentId = "cmt-8f3a1b",
  banEmail = false,
  banEmailDomain = false,
  banIP = false,
  deleteAllUsersComments = false,
  bannedUntil = "",
  isShadowBan = false,
  updateId = "",
  banReason = "",
  sso = ""
)
if response.isSome:
  let banResult = response.get()
  discard banResult
else:
  echo "No ban result returned"
[inline-code-end]

---