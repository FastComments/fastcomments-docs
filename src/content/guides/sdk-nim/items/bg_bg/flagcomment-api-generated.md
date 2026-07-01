## Параметри

| Име | Тип | Задължително | Описание |
|------|------|--------------|----------|
| tenantId | string | Да |  |
| id | string | Не |  |
| options | FlagCommentOptions | Не |  |

## Отговор

Връща: [`Option[FlagCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_response.nim)

## Пример

[inline-code-attrs-start title = 'flagComment Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let options = FlagCommentOptions(reason: "spam content", note: "Automated posting detected", isSpam: true, categories: @["spam"])
let (flagRes, httpRes) = client.flagComment(tenantId = "my-tenant-123", id = "cmt-789", options = options)
if flagRes.isSome:
  let res = flagRes.get()
  discard res
[inline-code-end]