## Parameters

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| text-search | string | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationSuggestResponse.swift)

## Пример

[inline-code-attrs-start title = 'getSearchSuggest Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следващите кодови примери все още са в бета. При каквито и да е проблеми, моля съобщете чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let textSearch = "textSearch_example" // String |  (по избор)
let sso = "sso_example" // String |  (по избор)

ModerationAPI.getSearchSuggest(tenantId: tenantId, options: ModerationAPI.GetSearchSuggestOptions(textSearch: textSearch, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]