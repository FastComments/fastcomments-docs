## Параметри

| Ім'я | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| commentId | string | Так |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`Option[GetCommentBanStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_ban_status_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getCommentBanStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentBanStatus(commentId = "cmt-987654321", sso = "")

if response.isSome:
  let banStatus = response.get()
  echo "Ban status for comment cmt-987654321: ", banStatus
else:
  echo "No ban status returned for comment cmt-987654321"
[inline-code-end]

---