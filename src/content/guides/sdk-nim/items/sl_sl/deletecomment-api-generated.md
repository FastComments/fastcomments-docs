## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |
| options | DeleteCommentOptions | Ne |  |

## Odgovor

Vrne: [`Option[DeleteCommentResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_comment_result.nim)

## Primer

[inline-code-attrs-start title = 'Primer deleteComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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