## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |
| sso | string | query | नहीं |  |
| includeUserInfo | string | query | नहीं | सेट करने पर, पेज को लाइक करने वाले उपयोगकर्ताओं को लौटाता है (अवतार और उपयोगकर्ता नाम)। केवल "public" स्वीकार किया जाता है, जो निजी प्रोफ़ाइलों को छोड़ देता है। |

## प्रतिक्रिया

लौटाता है: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV1PageLikes.swift)

## उदाहरण

[inline-code-attrs-start title = 'getV1PageLikes उदाहरण'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// निम्नलिखित कोड सैंपल अभी भी बीटा हैं। किसी भी समस्या के लिए, कृपया http://github.com/OpenAPITools/openapi-generator/issues/new पर रिपोर्ट करें
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (optional)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | सेट करने पर, पेज को लाइक करने वाले उपयोगकर्ताओं को लौटाता है (अवतार और उपयोगकर्ता नाम)। केवल \"public\" स्वीकार किया जाता है, जो निजी प्रोफ़ाइलों को छोड़ देता है। (optional)

PublicAPI.getV1PageLikes(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetV1PageLikesOptions(sso: sso, includeUserInfo: includeUserInfo)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]