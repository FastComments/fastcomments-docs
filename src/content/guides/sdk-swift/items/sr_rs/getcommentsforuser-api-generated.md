## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Не |  |
| direction | string | query | Не |  |
| repliesToUserId | string | query | Не |  |
| page | number | query | Не |  |
| includei10n | boolean | query | Не |  |
| locale | string | query | Не |  |
| isCrawler | boolean | query | Не |  |

## Одговор

Враћа: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsForUserResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getCommentsForUser'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примери кода још увек су у бета фази. За сваки проблем, пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let userId = "userId_example" // String |  (опционо)
let direction = SortDirections() // SortDirections |  (опционо)
let repliesToUserId = "repliesToUserId_example" // String |  (опционо)
let page = 987 // Double |  (опционо)
let includei10n = true // Bool |  (опционо)
let locale = "locale_example" // String |  (опционо)
let isCrawler = true // Bool |  (опционо)

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