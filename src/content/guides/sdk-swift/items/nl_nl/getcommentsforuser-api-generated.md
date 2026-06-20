## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| userId | string | query | Nee |  |
| direction | string | query | Nee |  |
| repliesToUserId | string | query | Nee |  |
| page | number | query | Nee |  |
| includei10n | boolean | query | Nee |  |
| locale | string | query | Nee |  |
| isCrawler | boolean | query | Nee |  |

## Respons

Retourneert: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsForUserResponse.swift)

## Voorbeeld

[inline-code-attrs-start title = 'getCommentsForUser Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Meld problemen via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let userId = "userId_example" // String |  (optioneel)
let direction = SortDirections() // SortDirections |  (optioneel)
let repliesToUserId = "repliesToUserId_example" // String |  (optioneel)
let page = 987 // Double |  (optioneel)
let includei10n = true // Bool |  (optioneel)
let locale = "locale_example" // String |  (optioneel)
let isCrawler = true // Bool |  (optioneel)

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