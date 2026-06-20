## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| userId | string | query | Nej |  |
| direction | string | query | Nej |  |
| repliesToUserId | string | query | Nej |  |
| page | number | query | Nej |  |
| includei10n | boolean | query | Nej |  |
| locale | string | query | Nej |  |
| isCrawler | boolean | query | Nej |  |

## Svar

Returnerer: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsForUserResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på getCommentsForUser'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. Ved problemer, rapportér venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let userId = "userId_example" // String |  (valgfri)
let direction = SortDirections() // SortDirections |  (valgfri)
let repliesToUserId = "repliesToUserId_example" // String |  (valgfri)
let page = 987 // Double |  (valgfri)
let includei10n = true // Bool |  (valgfri)
let locale = "locale_example" // String |  (valgfri)
let isCrawler = true // Bool |  (valgfri)

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