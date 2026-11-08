## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |
| sso | string | query | नहीं |  |
| includeUserInfo | string | query | नहीं | सेट होने पर, प्रत्येक react id के लिए उपयोगकर्ताओं को लौटाता है (अवतार और उपयोगकर्ता नाम)। केवल "public" स्वीकार किया जाता है, जो निजी प्रोफाइल को छोड़ देता है। |

## प्रतिक्रिया

लौटाता है: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV2PageReacts.swift)

## उदाहरण

[inline-code-attrs-start title = 'getV2PageReacts उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (optional)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | सेट होने पर, प्रत्येक react id के लिए उपयोगकर्ताओं को लौटाता है (अवतार और उपयोगकर्ता नाम)। केवल \"public\" स्वीकार किया जाता है, जो निजी प्रोफाइल को छोड़ देता है। (optional)

PublicAPI.getV2PageReacts(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetV2PageReactsOptions(sso: sso, includeUserInfo: includeUserInfo)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]