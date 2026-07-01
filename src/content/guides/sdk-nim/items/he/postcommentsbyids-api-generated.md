## פרמטרים

| שם | סוג | נחוץ | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| commentsByIdsParams | CommentsByIdsParams | לא |  |
| sso | string = "" | לא |  |

## תגובה

מחזיר: [`Option[ModerationAPIChildCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_child_comments_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמת postCommentsByIds'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let params = CommentsByIdsParams(commentIds = @["cmt-001", "cmt-002"])
let (maybeResp, httpResp) = client.postCommentsByIds(tenantId = tenantId, commentsByIdsParams = params, sso = "")
if maybeResp.isSome:
  let resp = maybeResp.get()
  # השתמש בתגובה לפי הצורך
[inline-code-end]

---