## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| namespace | string | path | 是 |  |
| component | string | path | 是 |  |
| locale | string | query | 否 |  |
| useFullTranslationIds | boolean | query | 否 |  |

## 回應

回傳: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTranslationsResponse.swift)

## 範例

[inline-code-attrs-start title = 'getTranslations 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列程式碼範例仍為測試版。如遇任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let namespace = "namespace_example" // String | 
let component = "component_example" // String | 
let locale = "locale_example" // String |  (可選)
let useFullTranslationIds = true // Bool |  (可選)

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