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

## Отговор

Връща: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsForUserResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getCommentsForUser'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следващите примери за код все още са в бета. За всякакъв проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let userId = "userId_example" // String |  (по избор)
let direction = SortDirections() // SortDirections |  (по избор)
let repliesToUserId = "repliesToUserId_example" // String |  (по избор)
let page = 987 // Double |  (по избор)
let includei10n = true // Bool |  (по избор)
let locale = "locale_example" // String |  (по избор)
let isCrawler = true // Bool |  (по избор)

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