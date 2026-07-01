## Параметри

| Назва | Тип | Розташування | Обов’язковий | Опис |
|------|------|----------|----------|-------------|
| userId | string | query | Ні |  |
| direction | string | query | Ні |  |
| repliesToUserId | string | query | Ні |  |
| page | number | query | Ні |  |
| includei10n | boolean | query | Ні |  |
| locale | string | query | Ні |  |
| isCrawler | boolean | query | Ні |  |

## Відповідь

Повертає: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsForUserResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getCommentsForUser'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні зразки коду все ще бета. При виникненні будь‑яких проблем, будь ласка, повідомте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let userId = "userId_example" // String |  (необов'язково)
let direction = SortDirections() // SortDirections |  (необов'язково)
let repliesToUserId = "repliesToUserId_example" // String |  (необов'язково)
let page = 987 // Double |  (необов'язково)
let includei10n = true // Bool |  (необов'язково)
let locale = "locale_example" // String |  (необов'язково)
let isCrawler = true // Bool |  (необов'язково)

PublicAPI.getCommentsForUser(options: PublicAPI.GetCommentsForUserOptions(userId: userId, direction: direction, repliesToUserId: repliesToUserId, page: page, includei10n: includei10n, locale: locale, isCrawler: isCrawler)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]