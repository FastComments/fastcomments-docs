---
## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| options | PostRestoreDeletedCommentOptions | Ні |  |

## Відповідь

Повертає: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Приклад

[inline-code-attrs-start title = 'postRestoreDeletedComment Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---