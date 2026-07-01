---
## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| sso | string = "" | No |  |

## תגובה

מחזירה: [`Option[GetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_text_response.nim)

## דוגמה

[inline-code-attrs-start title = 'getModerationCommentText דוגמה'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.getModerationCommentText(
  tenantId = "my-tenant-123",
  commentId = "comment-456abc",
  sso = ""
)

if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]

---