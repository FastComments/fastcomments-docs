## Parameters

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | PostRestoreDeletedCommentOptions | No |  |

## תגובה

מחזיר: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמת postRestoreDeletedComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.postRestoreDeletedComment(
  tenantId = "my-tenant-123",
  commentId = "comment-456",
  options = default(PostRestoreDeletedCommentOptions)
)

if respOpt.isSome:
  let empty = respOpt.get()
  echo "Comment restored"
[inline-code-end]