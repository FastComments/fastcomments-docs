## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| isFlagged | bool | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример на flagCommentPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.flagCommentPublic(
  tenantId = "my-tenant-123",
  commentId = "comment-98765",
  isFlagged = false,
  sso = ""
)
if response.isSome:
  let flagResult = response.get()
  discard flagResult
[inline-code-end]

---