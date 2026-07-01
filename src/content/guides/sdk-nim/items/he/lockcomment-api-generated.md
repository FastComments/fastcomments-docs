## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| commentId | string | כן |  |
| broadcastId | string | לא |  |
| sso | string = "" | לא |  |

## תגובה

מחזיר: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל‑lockComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (lockResult, httpRes) = client.lockComment(
  tenantId = "my-tenant-123",
  commentId = "comment-456",
  broadcastId = "",
  sso = "")

if lockResult.isSome:
  let resp = lockResult.get()
  discard resp
[inline-code-end]

---