## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Non |  |
| direction | string | query | Non |  |
| repliesToUserId | string | query | Non |  |
| page | number | query | Non |  |
| includei10n | boolean | query | Non |  |
| locale | string | query | Non |  |
| isCrawler | boolean | query | Non |  |

## Réponse

Renvoie : [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsForUserResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de getCommentsForUser'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let userId = "userId_example" // String |  (facultatif)
let direction = SortDirections() // SortDirections |  (facultatif)
let repliesToUserId = "repliesToUserId_example" // String |  (facultatif)
let page = 987 // Double |  (facultatif)
let includei10n = true // Bool |  (facultatif)
let locale = "locale_example" // String |  (facultatif)
let isCrawler = true // Bool |  (facultatif)

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