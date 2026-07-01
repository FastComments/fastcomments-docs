## Параметри

| Ім'я | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Ні |  |
| options | UnFlagCommentOptions | Ні |  |

## Відповідь

Повертає: [`Option[FlagCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_response.nim)

## Приклад

[inline-code-attrs-start title = 'unFlagComment Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (flagRespOpt, httpResp) = client.unFlagComment(tenantId = "my-tenant-123", id = "comment-456", options = UnFlagCommentOptions())
if flagRespOpt.isSome:
  let flagResp = flagRespOpt.get()
[inline-code-end]