## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| namespace | string | path | כן |  |
| component | string | path | כן |  |
| locale | string | query | לא |  |
| useFullTranslationIds | boolean | query | לא |  |

## תגובה

מחזיר: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTranslationsResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getTranslations'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד שלהלן עדיין בבטא. במקרה של בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let namespace = "namespace_example" // String | 
let component = "component_example" // String | 
let locale = "locale_example" // String |  (אופציונלי)
let useFullTranslationIds = true // Bool |  (אופציונלי)

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