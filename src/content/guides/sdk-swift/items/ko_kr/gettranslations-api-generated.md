## Parameters

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| namespace | string | path | 예 |  |
| component | string | path | 예 |  |
| locale | string | query | 아니오 |  |
| useFullTranslationIds | boolean | query | 아니오 |  |

## Response

Returns: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTranslationsResponse.swift)

## Example

[inline-code-attrs-start title = 'getTranslations 예시'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타 버전입니다. 문제 발생 시 http://github.com/OpenAPITools/openapi-generator/issues/new 로 보고해 주세요
import FastCommentsSwift

let namespace = "namespace_example" // String | 
let component = "component_example" // String | 
let locale = "locale_example" // String |  (옵션)
let useFullTranslationIds = true // Bool |  (옵션)

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