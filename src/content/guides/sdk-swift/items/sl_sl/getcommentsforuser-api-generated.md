## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| userId | string | query | Ne |  |
| direction | string | query | Ne |  |
| repliesToUserId | string | query | Ne |  |
| page | number | query | Ne |  |
| includei10n | boolean | query | Ne |  |
| locale | string | query | Ne |  |
| isCrawler | boolean | query | Ne |  |

## Odgovor

Vrne: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsForUserResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getCommentsForUser'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta fazi. Za morebitne težave jih prijavite preko http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let userId = "userId_example" // String |  (izbirno)
let direction = SortDirections() // SortDirections |  (izbirno)
let repliesToUserId = "repliesToUserId_example" // String |  (izbirno)
let page = 987 // Double |  (izbirno)
let includei10n = true // Bool |  (izbirno)
let locale = "locale_example" // String |  (izbirno)
let isCrawler = true // Bool |  (izbirno)

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