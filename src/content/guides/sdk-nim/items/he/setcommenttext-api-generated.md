## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| commentId | string | כן |  |
| broadcastId | string | לא |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | לא |  |
| editKey | string | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`Option[PublicAPISetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_api_set_comment_text_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-setCommentText'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.setCommentText(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  broadcastId = "",
  commentTextUpdateRequest = CommentTextUpdateRequest(text: "Updated comment text to fix a typo and clarify meaning."),
  editKey = "",
  sso = ""
)
if response.isSome:
  let result = response.get()
  discard result
[inline-code-end]