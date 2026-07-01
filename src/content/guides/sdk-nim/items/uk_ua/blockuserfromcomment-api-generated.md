## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| blockFromCommentParams | BlockFromCommentParams | No |  |
| options | BlockUserFromCommentOptions | No |  |

## Відповідь

Повертає: [`Option[BlockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_block_success.nim)

## Приклад

[inline-code-attrs-start title = 'blockUserFromComment Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = BlockFromCommentParams()
let opts = BlockUserFromCommentOptions()
let (blockResult, httpResp) = client.blockUserFromComment(
  tenantId = "my-tenant-123",
  id = "comment-456",
  blockFromCommentParams = params,
  options = opts
)
if blockResult.isSome:
  let success = blockResult.get()
  discard success
[inline-code-end]