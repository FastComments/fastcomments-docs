## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| namespace | string | path | Да |  |
| component | string | path | Да |  |
| locale | string | query | Не |  |
| useFullTranslationIds | boolean | query | Не |  |

## Отговор

Връща: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTranslationsResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getTranslations'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следващите примери на код все още са в бета. За всякакви проблеми, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let namespace = "namespace_example" // String | 
let component = "component_example" // String | 
let locale = "locale_example" // String |  (незадължително)
let useFullTranslationIds = true // Bool |  (незадължително)

PublicAPI.getTranslations(namespace: namespace, component: component, locale: locale, useFullTranslationIds: useFullTranslationIds) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]