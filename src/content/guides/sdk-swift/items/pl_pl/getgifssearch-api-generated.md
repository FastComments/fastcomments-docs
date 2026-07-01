## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|--------------|----------|------|
| tenantId | string | path | Tak |  |
| search | string | query | Tak |  |
| locale | string | query | Nie |  |
| rating | string | query | Nie |  |
| page | number | query | Nie |  |

## Odpowiedź

Zwraca: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetGifsSearchResponse.swift)

## Przykład

[inline-code-attrs-start title = 'getGifsSearch Przykład'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są wciąż w wersji beta. W razie jakichkolwiek problemów, proszę zgłosić je pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let search = "search_example" // String | 
let locale = "locale_example" // String |  (opcjonalny)
let rating = "rating_example" // String |  (opcjonalny)
let page = 987 // Double |  (opcjonalny)

PublicAPI.getGifsSearch(tenantId: tenantId, search: search, options: PublicAPI.GetGifsSearchOptions(locale: locale, rating: rating, page: page)) { (response, error) in
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