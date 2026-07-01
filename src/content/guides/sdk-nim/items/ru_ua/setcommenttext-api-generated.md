## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|--------------|------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | No |  |
| options | SetCommentTextOptions | No |  |

## Відповідь

Повертає: [`Option[PublicAPISetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_api_set_comment_text_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад setCommentText'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let commentUpdate = CommentTextUpdateRequest(text: "Updated comment text")
let opts = SetCommentTextOptions()
let (response, httpResponse) = client.setCommentText(
  tenantId = "my-tenant-123",
  commentId = "cmt-456",
  broadcastId = "broadcast-789",
  commentTextUpdateRequest = commentUpdate,
  options = opts)
if response.isSome:
  let result = response.get()
[inline-code-end]