## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| commentId | string | כן |  |
| broadcastId | string | לא |  |
| editKey | string | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`Option[DeleteCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_comment_public200response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-deleteCommentPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteCommentPublic(
  tenantId = "my-tenant-123",
  commentId = "cmt-987654",
  broadcastId = "",
  editKey = "",
  sso = ""
)
if response.isSome:
  let deleted = response.get()
  echo "Delete succeeded"
  echo "HTTP status: ", httpResponse.status
else:
  echo "Delete failed, HTTP status: ", httpResponse.status
[inline-code-end]

---