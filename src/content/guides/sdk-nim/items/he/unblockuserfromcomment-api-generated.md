## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | לא |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | לא |  |
| userId | string | לא |  |
| anonUserId | string | לא |  |

## תגובה

מחזיר: [`Option[UnblockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_unblock_success.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-unBlockUserFromComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unBlockUserFromComment(
  tenantId = "my-tenant-123",
  id = "comment-9f3b2a",
  unBlockFromCommentParams = UnBlockFromCommentParams(),
  userId = "user-1024",
  anonUserId = "anon-77b"
)

if response.isSome:
  let unblockResult = response.get()
  echo unblockResult
else:
  echo "Unblock failed"
[inline-code-end]

---