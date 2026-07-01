## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| broadcastId | string | Нет |  |
| options | DeleteCommentPublicOptions | Нет |  |

## Ответ

Возвращает: [`Option[PublicAPIDeleteCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_api_delete_comment_response.nim)

## Пример

[inline-code-attrs-start title = 'deleteCommentPublic Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (responseOpt, httpResp) = client.deleteCommentPublic(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  broadcastId = "",
  options = DeleteCommentPublicOptions())
if responseOpt.isSome:
  let resp = responseOpt.get()
  echo resp
[inline-code-end]