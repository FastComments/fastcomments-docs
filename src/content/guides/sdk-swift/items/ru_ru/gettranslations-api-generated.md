## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| namespace | string | path | Да |  |
| component | string | path | Да |  |
| locale | string | query | Нет |  |
| useFullTranslationIds | boolean | query | Нет |  |

## Ответ

Возвращает: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTranslationsResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getTranslations'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода всё ещё находятся в бета-версии. При возникновении проблем, пожалуйста, сообщите через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let namespace = "namespace_example" // String | 
let component = "component_example" // String | 
let locale = "locale_example" // String |  (необязательно)
let useFullTranslationIds = true // Bool |  (необязательно)

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

---