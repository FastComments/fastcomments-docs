## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |
| contextUserId | string | Ne |  |
| isLive | bool | Ne |  |

## Odgovor

Vraća: [`Option[DeleteCommentResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_comment_result.nim)

## Primer

[inline-code-attrs-start title = 'deleteComment Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteComment(tenantId = "my-tenant-123", id = "cmt-98765", contextUserId = "user-456", isLive = true)
if response.isSome:
  let result = response.get()
  echo "DeleteCommentResult received"
else:
  echo "No result, HTTP status: ", httpResponse.status
[inline-code-end]

---