## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentsByIdsParams | CommentsByIdsParams | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`Option[ModerationAPIChildCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_child_comments_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-postCommentsByIds'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let commentsParams = CommentsByIdsParams(ids = @["cmt-1001", "cmt-1002"], includeReplies = true, maxDepth = 2)
let (response, httpResponse) = client.postCommentsByIds(commentsByIdsParams = commentsParams, sso = "sso-user-7f3a9b")
if response.isSome:
  let result = response.get()
  echo "Received child comments response: ", result
[inline-code-end]

---