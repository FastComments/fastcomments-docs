## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| search | string | query | Да |  |
| locale | string | query | Не |  |
| rating | string | query | Не |  |
| page | number | query | Не |  |

## Отговор

Връща: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetGifsSearchResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getGifsSearch'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следните примери за код все още са в бета. При проблем моля докладвайте на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let search = "search_example" // String | 
let locale = "locale_example" // String |  (по избор)
let rating = "rating_example" // String |  (по избор)
let page = 987 // Double |  (по избор)

PublicAPI.getGifsSearch(tenantId: tenantId, search: search, locale: locale, rating: rating, page: page) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]