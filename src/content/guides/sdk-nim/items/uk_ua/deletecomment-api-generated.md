## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Ні |  |
| contextUserId | string | Ні |  |
| isLive | bool | Ні |  |

## Відповідь

Повертає: [`Option[DeleteComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_comment200response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteComment(tenantId = "my-tenant-123", id = "cmt-456abc", contextUserId = "user-789", isLive = true)
if response.isSome:
  let deleted = response.get()
  discard deleted
  echo "Delete succeeded"
else:
  echo "No delete response"
[inline-code-end]

---