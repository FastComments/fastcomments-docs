## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| namespace | string | path | हाँ |  |
| component | string | path | हाँ |  |
| locale | string | query | नहीं |  |
| useFullTranslationIds | boolean | query | नहीं |  |

## प्रतिक्रिया

रिटर्न: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTranslationsResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getTranslations उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा में हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let namespace = "namespace_example" // String | 
let component = "component_example" // String | 
let locale = "locale_example" // String |  (वैकल्पिक)
let useFullTranslationIds = true // Bool |  (वैकल्पिक)

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