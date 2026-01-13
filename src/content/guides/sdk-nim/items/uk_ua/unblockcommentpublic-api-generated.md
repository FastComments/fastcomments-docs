## Параметри

| Назва | Тип | Обов'язкове | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`Option[UnBlockCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_un_block_comment_public200response.nim)

## Приклад

[inline-code-attrs-start title = 'unBlockCommentPublic Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unBlockCommentPublic(
  tenantId = "news-site-456",
  commentId = "comment-abc123",
  publicBlockFromCommentParams = PublicBlockFromCommentParams{},
  sso = ""
)
if response.isSome:
  let unblocked = response.get()
  discard unblocked
else:
  discard httpResponse
[inline-code-end]

---