## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| commentId | string | כן |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`Option[GetCommentBanStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_ban_status_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getCommentBanStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentBanStatus(commentId = "cmt-987654321", sso = "")

if response.isSome:
  let banStatus = response.get()
  echo "Ban status for comment cmt-987654321: ", banStatus
else:
  echo "No ban status returned for comment cmt-987654321"
[inline-code-end]

---