## Параметри

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|----------|----------|-------------|
| namespace | string | path | Так |  |
| component | string | path | Так |  |
| locale | string | query | Ні |  |
| useFullTranslationIds | boolean | query | Ні |  |

## Відповідь

Повертає: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTranslationsResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getTranslations'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду все ще у бета-версії. У разі будь-яких проблем, будь ласка, повідомте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let namespace = "namespace_example" // String | 
let component = "component_example" // String | 
let locale = "locale_example" // String |  (необов’язково)
let useFullTranslationIds = true // Bool |  (необов’язково)

PublicAPI.getTranslations(namespace: namespace, component: component, options: PublicAPI.GetTranslationsOptions(locale: locale, useFullTranslationIds: useFullTranslationIds)) { (response, error) in
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