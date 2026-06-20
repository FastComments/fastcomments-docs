---
## Параметри

| Име | Тип | Потребно | Опис |
|------|------|----------|-------------|
| commentId | string | Да |  |
| spam | bool | Не |  |
| permNotSpam | bool | Не |  |
| sso | string | Не |  |

## Одговор

Враћа: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Пример

[inline-code-attrs-start title = 'postSetCommentSpamStatus Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postSetCommentSpamStatus(
  commentId = "cmt-20250619-842",
  spam = false,
  permNotSpam = false,
  sso = ""
)
if response.isSome:
  let apiEmpty = response.get()
  discard apiEmpty
[inline-code-end]

---