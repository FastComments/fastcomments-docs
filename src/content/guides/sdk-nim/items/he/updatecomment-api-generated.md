## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | לא |  |
| updatableCommentParams | UpdatableCommentParams | לא |  |
| contextUserId | string | לא |  |
| doSpamCheck | bool | לא |  |
| isLive | bool | לא |  |

## תגובה

מחזיר: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-updateComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updatableCommentParams = UpdatableCommentParams(content: "Fixed a typo in the second paragraph", tags: @["article-edit", "typo"], isApproved: true)
let (response, httpResponse) = client.updateComment(
  tenantId = "my-tenant-123",
  id = "comment-456",
  updatableCommentParams = updatableCommentParams,
  contextUserId = "user-789",
  doSpamCheck = true,
  isLive = true
)
if response.isSome:
  let flagResp = response.get()
  discard flagResp
[inline-code-end]

---