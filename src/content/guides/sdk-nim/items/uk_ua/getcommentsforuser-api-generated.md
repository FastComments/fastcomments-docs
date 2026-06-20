## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| userId | string | Ні |  |
| direction | SortDirections | Ні |  |
| repliesToUserId | string | Ні |  |
| page | float64 | Ні |  |
| includei10n | bool | Ні |  |
| locale | string | Ні |  |
| isCrawler | bool | Ні |  |

## Відповідь

Повертає: [`Option[GetCommentsForUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_for_user_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getCommentsForUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentsForUser(
  userId = "user-8421",
  direction = SortDirections.Newest,
  repliesToUserId = "",
  page = 1.0,
  includei10n = true,
  locale = "en-US",
  isCrawler = false
)

if response.isSome:
  let comments = response.get()
  discard comments
[inline-code-end]

---