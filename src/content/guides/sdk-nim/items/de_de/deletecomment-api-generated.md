## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| options | DeleteCommentOptions | No |  |

## Antwort

Rückgabe: [`Option[DeleteCommentResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_comment_result.nim)

## Beispiel

[inline-code-attrs-start title = 'deleteComment Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (delResult, httpResponse) = client.deleteComment(
  tenantId = "my-tenant-123",
  id = "comment-456",
  options = DeleteCommentOptions()
)

if delResult.isSome:
  let result = delResult.get()
  echo result
[inline-code-end]