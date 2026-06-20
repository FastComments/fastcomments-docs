## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Ні |  |
| contextUserId | string | Ні |  |
| isLive | bool | Ні |  |

## Відповідь

Повертає: [`Option[DeleteCommentResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_comment_result.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад використання deleteComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteComment(tenantId = "my-tenant-123", id = "cmt-98765", contextUserId = "user-456", isLive = true)
if response.isSome:
  let result = response.get()
  echo "DeleteCommentResult received"
else:
  echo "No result, HTTP status: ", httpResponse.status
[inline-code-end]

---