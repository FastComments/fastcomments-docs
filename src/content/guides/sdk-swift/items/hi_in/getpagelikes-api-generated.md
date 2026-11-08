किसी पेज के लाइक की संख्या, वैकल्पिक रूप से उन उपयोगकर्ताओं के साथ जिन्होंने इसे लाइक किया।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| urlId | string | query | हाँ |  |
| includeUserInfo | string | query | नहीं | "public" निजी प्रोफ़ाइल वाले उपयोगकर्ताओं को छोड़ देता है, "include-private" उन्हें शामिल करता है। |

## प्रतिक्रिया

रिटर्न करता है: [`GetPageLikesAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPageLikesAPIResponse.swift)

## उदाहरण

[inline-code-attrs-start title = 'getPageLikes उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | \"public\" निजी प्रोफ़ाइल वाले उपयोगकर्ताओं को छोड़ देता है, \"include-private\" उन्हें शामिल करता है। (optional)

DefaultAPI.getPageLikes(tenantId: tenantId, urlId: urlId, includeUserInfo: includeUserInfo) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]