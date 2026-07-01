## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|-------------|----------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| setCommentTextParams | SetCommentTextParams | No |  |
| options | PostSetCommentTextOptions | No |  |

## Ответ

Возвращает: [`Option[SetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_comment_text_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример postSetCommentText'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (responseOpt, httpResponse) = client.postSetCommentText(
  tenantId = "my-tenant-123",
  commentId = "comment-987654",
  setCommentTextParams = SetCommentTextParams(),
  options = PostSetCommentTextOptions()
)

if responseOpt.isSome:
  let updatedComment = responseOpt.get()
[inline-code-end]