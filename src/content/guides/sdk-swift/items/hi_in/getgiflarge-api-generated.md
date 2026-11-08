## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| largeInternalURLSanitized | string | query | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GifGetLargeResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getGifLarge उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा में हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let largeInternalURLSanitized = "largeInternalURLSanitized_example" // String | 

PublicAPI.getGifLarge(tenantId: tenantId, largeInternalURLSanitized: largeInternalURLSanitized) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]