## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | לא |  |

## תגובה

מחזיר: [`Option[APIGetCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_comment_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getComment(tenantId = "my-tenant-123", id = "cmt-7890")
if response.isSome:
  let comment = response.get()
  discard comment
else:
  echo "Comment not found"
[inline-code-end]

---