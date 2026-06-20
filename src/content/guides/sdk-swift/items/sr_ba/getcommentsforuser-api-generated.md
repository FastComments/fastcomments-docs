## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Ne |  |
| direction | string | query | Ne |  |
| repliesToUserId | string | query | Ne |  |
| page | number | query | Ne |  |
| includei10n | boolean | query | Ne |  |
| locale | string | query | Ne |  |
| isCrawler | boolean | query | Ne |  |

## Odgovor

Vraća: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsForUserResponse.swift)

## Primjer

[inline-code-attrs-start title = 'getCommentsForUser Primjer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta fazi. Za bilo koji problem, prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let userId = "userId_example" // String |  (opciono)
let direction = SortDirections() // SortDirections |  (opciono)
let repliesToUserId = "repliesToUserId_example" // String |  (opciono)
let page = 987 // Double |  (opciono)
let includei10n = true // Bool |  (opciono)
let locale = "locale_example" // String |  (opciono)
let isCrawler = true // Bool |  (opciono)

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