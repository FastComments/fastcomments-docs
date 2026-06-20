## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| userId | string | query | Nein |  |
| direction | string | query | Nein |  |
| repliesToUserId | string | query | Nein |  |
| page | number | query | Nein |  |
| includei10n | boolean | query | Nein |  |
| locale | string | query | Nein |  |
| isCrawler | boolean | query | Nein |  |

## Antwort

Gibt zurück: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsForUserResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'getCommentsForUser Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch Beta. Bei Problemen melden Sie sich bitte über http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let userId = "userId_example" // String |  (optional)
let direction = SortDirections() // SortDirections |  (optional)
let repliesToUserId = "repliesToUserId_example" // String |  (optional)
let page = 987 // Double |  (optional)
let includei10n = true // Bool |  (optional)
let locale = "locale_example" // String |  (optional)
let isCrawler = true // Bool |  (optional)

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