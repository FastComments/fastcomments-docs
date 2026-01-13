## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | לא |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | לא |  |
| userId | string | לא |  |
| anonUserId | string | לא |  |

## תגובה

מחזיר: [`Option[UnBlockCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_un_block_comment_public200response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-unBlockUserFromComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unBlockUserFromComment(
  tenantId = "news-site-001",
  id = "cmt-8fj3k9",
  unBlockFromCommentParams = UnBlockFromCommentParams(),
  userId = "user-98765",
  anonUserId = ""
)

if response.isSome:
  let unblocked = response.get()
  discard unblocked
[inline-code-end]

---