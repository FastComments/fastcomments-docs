## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| commentId | string | כן |  |
| sso | string = "" | לא |  |

## תגובה

מחזיר: [`Option[GetCommentBanStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_ban_status_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה getCommentBanStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (banStatusOpt, httpResp) = client.getCommentBanStatus(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  sso = "")

if banStatusOpt.isSome:
  let banStatus = banStatusOpt.get()
  echo banStatus
[inline-code-end]