## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| search | string | query | Так |  |
| locale | string | query | Ні |  |
| rating | string | query | Ні |  |
| page | number | query | Ні |  |

## Response

Повертає: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetGifsSearchResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getGifsSearch'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду все ще в бета-версії. У разі будь-якої проблеми, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let search = "search_example" // String | 
let locale = "locale_example" // String |  (необов'язково)
let rating = "rating_example" // String |  (необов'язково)
let page = 987 // Double |  (необов'язково)

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