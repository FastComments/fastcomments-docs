## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| userId | string | query | Нет |  |
| direction | string | query | Нет |  |
| repliesToUserId | string | query | Нет |  |
| page | number | query | Нет |  |
| includei10n | boolean | query | Нет |  |
| locale | string | query | Нет |  |
| isCrawler | boolean | query | Нет |  |

## Ответ

Возвращает: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsForUserResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getCommentsForUser'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода всё ещё находятся в бета-версии. При возникновении проблем, пожалуйста, сообщите через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let userId = "userId_example" // String |  (необязательно)
let direction = SortDirections() // SortDirections |  (необязательно)
let repliesToUserId = "repliesToUserId_example" // String |  (необязательно)
let page = 987 // Double |  (необязательно)
let includei10n = true // Bool |  (необязательно)
let locale = "locale_example" // String |  (необязательно)
let isCrawler = true // Bool |  (необязательно)

PublicAPI.getCommentsForUser(userId: userId, direction: direction, repliesToUserId: repliesToUserId, page: page, includei10n: includei10n, locale: locale, isCrawler: isCrawler) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---