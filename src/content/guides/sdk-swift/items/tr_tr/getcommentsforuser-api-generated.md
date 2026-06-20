## Parametreler

| İsim | Tip | Yer | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| userId | string | query | Hayır |  |
| direction | string | query | Hayır |  |
| repliesToUserId | string | query | Hayır |  |
| page | number | query | Hayır |  |
| includei10n | boolean | query | Hayır |  |
| locale | string | query | Hayır |  |
| isCrawler | boolean | query | Hayır |  |

## Yanıt

Döndürür: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsForUserResponse.swift)

## Örnek

[inline-code-attrs-start title = 'getCommentsForUser Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new üzerinden bildirin
import FastCommentsSwift

let userId = "userId_example" // String |  (isteğe bağlı)
let direction = SortDirections() // SortDirections |  (isteğe bağlı)
let repliesToUserId = "repliesToUserId_example" // String |  (isteğe bağlı)
let page = 987 // Double |  (isteğe bağlı)
let includei10n = true // Bool |  (isteğe bağlı)
let locale = "locale_example" // String |  (isteğe bağlı)
let isCrawler = true // Bool |  (isteğe bağlı)

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