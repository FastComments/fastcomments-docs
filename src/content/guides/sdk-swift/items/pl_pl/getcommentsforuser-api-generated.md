## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
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
// Następujące przykłady kodu są wciąż w wersji beta. W przypadku jakichkolwiek problemów, prosimy zgłaszać je pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let userId = "userId_example" // String |  (opcjonalnie)
let direction = SortDirections() // SortDirections |  (opcjonalnie)
let repliesToUserId = "repliesToUserId_example" // String |  (opcjonalnie)
let page = 987 // Double |  (opcjonalnie)
let includei10n = true // Bool |  (opcjonalnie)
let locale = "locale_example" // String |  (opcjonalnie)
let isCrawler = true // Bool |  (opcjonalnie)

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

---