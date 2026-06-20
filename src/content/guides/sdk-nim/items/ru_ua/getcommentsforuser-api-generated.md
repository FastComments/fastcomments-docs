## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| userId | string | Нет |  |
| direction | SortDirections | Нет |  |
| repliesToUserId | string | Нет |  |
| page | float64 | Нет |  |
| includei10n | bool | Нет |  |
| locale | string | Нет |  |
| isCrawler | bool | Нет |  |

## Ответ

Возвращает: [`Option[GetCommentsForUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_for_user_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getCommentsForUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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