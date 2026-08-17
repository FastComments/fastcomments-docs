## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| search | string | query | Да |  |
| locale | string | query | Не |  |
| rating | string | query | Не |  |
| page | number | query | Не |  |

## Одговор

Враћа: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetGifsSearchResponse.swift)

## Пример

[inline-code-attrs-start title = 'Primer getGifsSearch'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su još u beta fazi. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let search = "search_example" // String | 
let locale = "locale_example" // String |  (опционално)
let rating = "rating_example" // String |  (опционално)
let page = 987 // Double |  (опционално)

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