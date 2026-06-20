## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Нет |  |
| blockFromCommentParams | BlockFromCommentParams | Нет |  |
| userId | string | Нет |  |
| anonUserId | string | Нет |  |

## Ответ

Возвращает: [`Option[BlockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_block_success.nim)

## Пример

[inline-code-attrs-start title = 'Пример blockUserFromComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.blockUserFromComment(
  tenantId = "my-tenant-123",
  id = "cmt-7890",
  blockFromCommentParams = BlockFromCommentParams(
    reason = "Repeated abusive language",
    durationMinutes = 1440,
    notifyUser = true,
    tags = @["abuse", "automated"]
  ),
  userId = "user-456",
  anonUserId = ""
)

if response.isSome:
  let result = response.get()
  discard result
else:
  discard httpResponse
[inline-code-end]