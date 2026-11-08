किसी पेज के लिए कस्टम रिएक्ट काउंट्स, वैकल्पिक रूप से रिएक्ट करने वाले उपयोगकर्ताओं के साथ।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| urlId | string | query | हाँ |  |
| includeUserInfo | string | query | नहीं | "public" प्राइवेट प्रोफ़ाइल वाले उपयोगकर्ताओं को छोड़ देता है, "include-private" उन्हें शामिल करता है। |

## प्रतिक्रिया

रिटर्न करता है: [`GetPageReactsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPageReactsAPIResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getPageReacts उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | \"public\" प्राइवेट प्रोफ़ाइल वाले उपयोगकर्ताओं को छोड़ देता है, \"include-private\" उन्हें शामिल करता है। (optional)

DefaultAPI.getPageReacts(tenantId: tenantId, urlId: urlId, includeUserInfo: includeUserInfo) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]