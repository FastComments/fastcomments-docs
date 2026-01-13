## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | לא |  |
| editKey | string | לא |  |

## תגובה

מחזיר: [`Option[DeleteCommentVote_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_comment_vote200response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-deleteVote'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteVote(tenantId = "my-tenant-123", id = "", editKey = "")
if response.isSome:
  let deleted = response.get()
  discard deleted
[inline-code-end]

---