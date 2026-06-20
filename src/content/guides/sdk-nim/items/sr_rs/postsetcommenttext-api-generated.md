## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| commentId | string | Да |  |
| setCommentTextParams | SetCommentTextParams | Не |  |
| sso | string | Не |  |

## Одговор

Враћа: [`Option[SetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_comment_text_response.nim)

## Пример

[inline-code-attrs-start title = 'postSetCommentText Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postSetCommentText(commentId = "comment-4821",
  setCommentTextParams = SetCommentTextParams(text = "Updated comment to clarify the main point and fix a typo."),
  sso = "sso-user-8f3b9c")

if response.isSome:
  let setCommentResp = response.get()
  echo "Received SetCommentTextResponse"
[inline-code-end]

---