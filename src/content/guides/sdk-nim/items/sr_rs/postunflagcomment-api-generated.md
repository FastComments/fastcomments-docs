## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| commentId | string | Да |  |
| sso | string | Не |  |

## Одговор

Враћа: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Пример

[inline-code-attrs-start title = 'postUnFlagComment пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postUnFlagComment(commentId = "comment-8f3a2b4e", sso = "")
if response.isSome:
  let apiEmpty = response.get()
  echo "Comment unflagged successfully, response: ", apiEmpty
else:
  echo "Failed to unflag comment. HTTP response: ", httpResponse
[inline-code-end]

---