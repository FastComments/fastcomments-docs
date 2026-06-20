## Parametry

| Name | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| userId | string | query | Nie |  |
| direction | string | query | Nie |  |
| repliesToUserId | string | query | Nie |  |
| page | number | query | Nie |  |
| includei10n | boolean | query | Nie |  |
| locale | string | query | Nie |  |
| isCrawler | boolean | query | Nie |  |

## Odpowiedź

Zwraca: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsForUserResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getCommentsForUser'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są nadal w wersji beta. W przypadku problemów zgłoś je przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let userId = "userId_example" // String |  (opcjonalne)
let direction = SortDirections() // SortDirections |  (opcjonalne)
let repliesToUserId = "repliesToUserId_example" // String |  (opcjonalne)
let page = 987 // Double |  (opcjonalne)
let includei10n = true // Bool |  (opcjonalne)
let locale = "locale_example" // String |  (opcjonalne)
let isCrawler = true // Bool |  (opcjonalne)

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