## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| commentId | string | כן |  |
| banEmail | bool | לא |  |
| banEmailDomain | bool | לא |  |
| banIP | bool | לא |  |
| deleteAllUsersComments | bool | לא |  |
| bannedUntil | string | לא |  |
| isShadowBan | bool | לא |  |
| updateId | string | לא |  |
| banReason | string | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`Option[BanUserFromCommentResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_ban_user_from_comment_result.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-postBanUserFromComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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