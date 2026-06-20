## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| commentId | string | Так |  |
| includeEmail | bool | Ні |  |
| includeIP | bool | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`Option[ModerationAPICommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_comment_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getModerationComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getModerationComment(commentId = "cmt-8f3b2a9d", includeEmail = false, includeIP = false, sso = "")
if response.isSome:
  let comment = response.get()
  discard comment
else:
  echo "No moderation comment returned"
[inline-code-end]

---